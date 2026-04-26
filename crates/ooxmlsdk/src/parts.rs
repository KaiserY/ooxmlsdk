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
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PartRef {
  AlternativeFormatImportPart(
        crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    ),
    CalculationChainPart(crate::parts::calculation_chain_part::CalculationChainPart),
    CellMetadataPart(crate::parts::cell_metadata_part::CellMetadataPart),
    #[cfg(feature = "microsoft365")]
    ChartColorStylePart(crate::parts::chart_color_style_part::ChartColorStylePart),
    ChartDrawingPart(crate::parts::chart_drawing_part::ChartDrawingPart),
    ChartPart(crate::parts::chart_part::ChartPart),
    #[cfg(feature = "microsoft365")]
    ChartStylePart(crate::parts::chart_style_part::ChartStylePart),
    ChartsheetPart(crate::parts::chartsheet_part::ChartsheetPart),
    CommentAuthorsPart(crate::parts::comment_authors_part::CommentAuthorsPart),
    ConnectionsPart(crate::parts::connections_part::ConnectionsPart),
    #[cfg(feature = "microsoft365")]
    ControlPropertiesPart(crate::parts::control_properties_part::ControlPropertiesPart),
    CoreFilePropertiesPart(
        crate::parts::core_file_properties_part::CoreFilePropertiesPart,
    ),
    #[cfg(feature = "microsoft365")]
    CustomDataPart(crate::parts::custom_data_part::CustomDataPart),
    #[cfg(feature = "microsoft365")]
    CustomDataPropertiesPart(
        crate::parts::custom_data_properties_part::CustomDataPropertiesPart,
    ),
    CustomFilePropertiesPart(
        crate::parts::custom_file_properties_part::CustomFilePropertiesPart,
    ),
    CustomPropertyPart(crate::parts::custom_property_part::CustomPropertyPart),
    CustomXmlMappingsPart(crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart),
    CustomXmlPart(crate::parts::custom_xml_part::CustomXmlPart),
    CustomXmlPropertiesPart(
        crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart,
    ),
    CustomizationPart(crate::parts::customization_part::CustomizationPart),
    DiagramColorsPart(crate::parts::diagram_colors_part::DiagramColorsPart),
    DiagramDataPart(crate::parts::diagram_data_part::DiagramDataPart),
    DiagramLayoutDefinitionPart(
        crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    ),
    #[cfg(feature = "microsoft365")]
    DiagramPersistLayoutPart(
        crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    ),
    DiagramStylePart(crate::parts::diagram_style_part::DiagramStylePart),
    DialogsheetPart(crate::parts::dialogsheet_part::DialogsheetPart),
    DigitalSignatureOriginPart(
        crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
    ),
    DocumentSettingsPart(crate::parts::document_settings_part::DocumentSettingsPart),
    #[cfg(feature = "microsoft365")]
    DocumentTasksPart(crate::parts::document_tasks_part::DocumentTasksPart),
    DrawingsPart(crate::parts::drawings_part::DrawingsPart),
    EmbeddedControlPersistenceBinaryDataPart(
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    ),
    EmbeddedControlPersistencePart(
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    ),
    EmbeddedObjectPart(crate::parts::embedded_object_part::EmbeddedObjectPart),
    EmbeddedPackagePart(crate::parts::embedded_package_part::EmbeddedPackagePart),
    EndnotesPart(crate::parts::endnotes_part::EndnotesPart),
    ExcelAttachedToolbarsPart(
        crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart,
    ),
    #[cfg(feature = "microsoft365")]
    ExtendedChartPart(crate::parts::extended_chart_part::ExtendedChartPart),
    ExtendedFilePropertiesPart(
        crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
    ),
    ExternalWorkbookPart(crate::parts::external_workbook_part::ExternalWorkbookPart),
    #[cfg(feature = "microsoft365")]
    FeaturePropertyBagsPart(
        crate::parts::feature_property_bags_part::FeaturePropertyBagsPart,
    ),
    FontPart(crate::parts::font_part::FontPart),
    FontTablePart(crate::parts::font_table_part::FontTablePart),
    FooterPart(crate::parts::footer_part::FooterPart),
    FootnotesPart(crate::parts::footnotes_part::FootnotesPart),
    GlossaryDocumentPart(crate::parts::glossary_document_part::GlossaryDocumentPart),
    HandoutMasterPart(crate::parts::handout_master_part::HandoutMasterPart),
    HeaderPart(crate::parts::header_part::HeaderPart),
    ImagePart(crate::parts::image_part::ImagePart),
    InternationalMacroSheetPart(
        crate::parts::international_macro_sheet_part::InternationalMacroSheetPart,
    ),
    #[cfg(feature = "microsoft365")]
    LabelInfoPart(crate::parts::label_info_part::LabelInfoPart),
    LegacyDiagramTextInfoPart(
        crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
    ),
    LegacyDiagramTextPart(crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart),
    MacroSheetPart(crate::parts::macro_sheet_part::MacroSheetPart),
    MailMergeRecipientDataPart(
        crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    ),
    MainDocumentPart(crate::parts::main_document_part::MainDocumentPart),
    #[cfg(feature = "microsoft365")]
    Model3DReferenceRelationshipPart(
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    ),
    #[cfg(feature = "microsoft365")]
    NamedSheetViewsPart(crate::parts::named_sheet_views_part::NamedSheetViewsPart),
    NotesMasterPart(crate::parts::notes_master_part::NotesMasterPart),
    NotesSlidePart(crate::parts::notes_slide_part::NotesSlidePart),
    NumberingDefinitionsPart(
        crate::parts::numbering_definitions_part::NumberingDefinitionsPart,
    ),
    PivotTableCacheDefinitionPart(
        crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
    ),
    PivotTableCacheRecordsPart(
        crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart,
    ),
    PivotTablePart(crate::parts::pivot_table_part::PivotTablePart),
    #[cfg(feature = "microsoft365")]
    PowerPointAuthorsPart(crate::parts::power_point_authors_part::PowerPointAuthorsPart),
    #[cfg(feature = "microsoft365")]
    PowerPointCommentPart(crate::parts::power_point_comment_part::PowerPointCommentPart),
    PresentationPart(crate::parts::presentation_part::PresentationPart),
    PresentationPropertiesPart(
        crate::parts::presentation_properties_part::PresentationPropertiesPart,
    ),
    QueryTablePart(crate::parts::query_table_part::QueryTablePart),
    QuickAccessToolbarCustomizationsPart(
        crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    ),
    #[cfg(feature = "microsoft365")]
    RdArrayPart(crate::parts::rd_array_part::RdArrayPart),
    #[cfg(feature = "microsoft365")]
    RdRichValuePart(crate::parts::rd_rich_value_part::RdRichValuePart),
    #[cfg(feature = "microsoft365")]
    RdRichValueStructurePart(
        crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart,
    ),
    #[cfg(feature = "microsoft365")]
    RdRichValueTypesPart(crate::parts::rd_rich_value_types_part::RdRichValueTypesPart),
    #[cfg(feature = "microsoft365")]
    RdRichValueWebImagePart(
        crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart,
    ),
    #[cfg(feature = "microsoft365")]
    RdSupportingPropertyBagPart(
        crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart,
    ),
    #[cfg(feature = "microsoft365")]
    RdSupportingPropertyBagStructurePart(
        crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart,
    ),
    #[cfg(feature = "microsoft365")]
    RibbonAndBackstageCustomizationsPart(
        crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    ),
    RibbonExtensibilityPart(
        crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart,
    ),
    #[cfg(feature = "microsoft365")]
    RichStylesPart(crate::parts::rich_styles_part::RichStylesPart),
    SharedStringTablePart(crate::parts::shared_string_table_part::SharedStringTablePart),
    SingleCellTablePart(crate::parts::single_cell_table_part::SingleCellTablePart),
    #[cfg(feature = "microsoft365")]
    SlicerCachePart(crate::parts::slicer_cache_part::SlicerCachePart),
    #[cfg(feature = "microsoft365")]
    SlicersPart(crate::parts::slicers_part::SlicersPart),
    SlideCommentsPart(crate::parts::slide_comments_part::SlideCommentsPart),
    SlideLayoutPart(crate::parts::slide_layout_part::SlideLayoutPart),
    SlideMasterPart(crate::parts::slide_master_part::SlideMasterPart),
    SlidePart(crate::parts::slide_part::SlidePart),
    SlideSyncDataPart(crate::parts::slide_sync_data_part::SlideSyncDataPart),
    SpreadsheetPrinterSettingsPart(
        crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart,
    ),
    StyleDefinitionsPart(crate::parts::style_definitions_part::StyleDefinitionsPart),
    #[cfg(feature = "microsoft365")]
    StylesWithEffectsPart(crate::parts::styles_with_effects_part::StylesWithEffectsPart),
    TableDefinitionPart(crate::parts::table_definition_part::TableDefinitionPart),
    TableStylesPart(crate::parts::table_styles_part::TableStylesPart),
    ThemeOverridePart(crate::parts::theme_override_part::ThemeOverridePart),
    ThemePart(crate::parts::theme_part::ThemePart),
    ThumbnailPart(crate::parts::thumbnail_part::ThumbnailPart),
    #[cfg(feature = "microsoft365")]
    TimeLineCachePart(crate::parts::time_line_cache_part::TimeLineCachePart),
    #[cfg(feature = "microsoft365")]
    TimeLinePart(crate::parts::time_line_part::TimeLinePart),
    UserDefinedTagsPart(crate::parts::user_defined_tags_part::UserDefinedTagsPart),
    VbaDataPart(crate::parts::vba_data_part::VbaDataPart),
    VbaProjectPart(crate::parts::vba_project_part::VbaProjectPart),
    ViewPropertiesPart(crate::parts::view_properties_part::ViewPropertiesPart),
    VmlDrawingPart(crate::parts::vml_drawing_part::VmlDrawingPart),
    VolatileDependenciesPart(
        crate::parts::volatile_dependencies_part::VolatileDependenciesPart,
    ),
    #[cfg(feature = "microsoft365")]
    WebExTaskpanesPart(crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart),
    #[cfg(feature = "microsoft365")]
    WebExtensionPart(crate::parts::web_extension_part::WebExtensionPart),
    WebSettingsPart(crate::parts::web_settings_part::WebSettingsPart),
    WordAttachedToolbarsPart(
        crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart,
    ),
    #[cfg(feature = "microsoft365")]
    WordCommentsExtensiblePart(
        crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
    ),
    #[cfg(feature = "microsoft365")]
    WordprocessingCommentsExPart(
        crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
    ),
    #[cfg(feature = "microsoft365")]
    WordprocessingCommentsIdsPart(
        crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
    ),
    WordprocessingCommentsPart(
        crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
    ),
    #[cfg(feature = "microsoft365")]
    WordprocessingPeoplePart(
        crate::parts::wordprocessing_people_part::WordprocessingPeoplePart,
    ),
    WordprocessingPrinterSettingsPart(
        crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
    ),
    WorkbookPart(crate::parts::workbook_part::WorkbookPart),
    #[cfg(feature = "microsoft365")]
    WorkbookPersonPart(crate::parts::workbook_person_part::WorkbookPersonPart),
    WorkbookRevisionHeaderPart(
        crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
    ),
    WorkbookRevisionLogPart(
        crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart,
    ),
    WorkbookStylesPart(crate::parts::workbook_styles_part::WorkbookStylesPart),
    WorkbookUserDataPart(crate::parts::workbook_user_data_part::WorkbookUserDataPart),
    WorksheetCommentsPart(crate::parts::worksheet_comments_part::WorksheetCommentsPart),
    WorksheetPart(crate::parts::worksheet_part::WorksheetPart),
    WorksheetSortMapPart(crate::parts::worksheet_sort_map_part::WorksheetSortMapPart),
    #[cfg(feature = "microsoft365")]
    WorksheetThreadedCommentsPart(
        crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart,
    ),
    XmlSignaturePart(crate::parts::xml_signature_part::XmlSignaturePart),
    ExtendedPart(crate::parts::extended_part::ExtendedPart),
}
pub trait PartRefDowncast: crate::sdk::SdkPartHandle {
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self>;
}
impl PartRefDowncast for crate::parts::alternative_format_import_part::AlternativeFormatImportPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::AlternativeFormatImportPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::calculation_chain_part::CalculationChainPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CalculationChainPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::cell_metadata_part::CellMetadataPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CellMetadataPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::chart_color_style_part::ChartColorStylePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ChartColorStylePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::chart_drawing_part::ChartDrawingPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ChartDrawingPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::chart_part::ChartPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ChartPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::chart_style_part::ChartStylePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ChartStylePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::chartsheet_part::ChartsheetPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ChartsheetPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::comment_authors_part::CommentAuthorsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CommentAuthorsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::connections_part::ConnectionsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ConnectionsPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::control_properties_part::ControlPropertiesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ControlPropertiesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::core_file_properties_part::CoreFilePropertiesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CoreFilePropertiesPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::custom_data_part::CustomDataPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CustomDataPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::custom_data_properties_part::CustomDataPropertiesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CustomDataPropertiesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::custom_file_properties_part::CustomFilePropertiesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CustomFilePropertiesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::custom_property_part::CustomPropertyPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CustomPropertyPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CustomXmlMappingsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::custom_xml_part::CustomXmlPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CustomXmlPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CustomXmlPropertiesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::customization_part::CustomizationPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::CustomizationPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::diagram_colors_part::DiagramColorsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::DiagramColorsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::diagram_data_part::DiagramDataPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::DiagramDataPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::DiagramLayoutDefinitionPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::DiagramPersistLayoutPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::diagram_style_part::DiagramStylePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::DiagramStylePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::dialogsheet_part::DialogsheetPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::DialogsheetPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::DigitalSignatureOriginPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::document_settings_part::DocumentSettingsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::DocumentSettingsPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::document_tasks_part::DocumentTasksPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::DocumentTasksPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::drawings_part::DrawingsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::DrawingsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast
for crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart {
    #[inline]
    fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
        match part_ref {
            PartRef::EmbeddedControlPersistenceBinaryDataPart(part) => Some(part),
            _ => None,
        }
    }
}
impl PartRefDowncast
  for crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::EmbeddedControlPersistencePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::embedded_object_part::EmbeddedObjectPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::EmbeddedObjectPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::embedded_package_part::EmbeddedPackagePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::EmbeddedPackagePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::endnotes_part::EndnotesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::EndnotesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ExcelAttachedToolbarsPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::extended_chart_part::ExtendedChartPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ExtendedChartPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ExtendedFilePropertiesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::external_workbook_part::ExternalWorkbookPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ExternalWorkbookPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::feature_property_bags_part::FeaturePropertyBagsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::FeaturePropertyBagsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::font_part::FontPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::FontPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::font_table_part::FontTablePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::FontTablePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::footer_part::FooterPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::FooterPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::footnotes_part::FootnotesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::FootnotesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::glossary_document_part::GlossaryDocumentPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::GlossaryDocumentPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::handout_master_part::HandoutMasterPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::HandoutMasterPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::header_part::HeaderPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::HeaderPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::image_part::ImagePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ImagePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::international_macro_sheet_part::InternationalMacroSheetPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::InternationalMacroSheetPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::label_info_part::LabelInfoPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::LabelInfoPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::LegacyDiagramTextInfoPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::LegacyDiagramTextPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::macro_sheet_part::MacroSheetPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::MacroSheetPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::MailMergeRecipientDataPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::main_document_part::MainDocumentPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::MainDocumentPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast
  for crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::Model3DReferenceRelationshipPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::named_sheet_views_part::NamedSheetViewsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::NamedSheetViewsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::notes_master_part::NotesMasterPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::NotesMasterPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::notes_slide_part::NotesSlidePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::NotesSlidePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::numbering_definitions_part::NumberingDefinitionsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::NumberingDefinitionsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast
  for crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::PivotTableCacheDefinitionPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::PivotTableCacheRecordsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::pivot_table_part::PivotTablePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::PivotTablePart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::power_point_authors_part::PowerPointAuthorsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::PowerPointAuthorsPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::power_point_comment_part::PowerPointCommentPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::PowerPointCommentPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::presentation_part::PresentationPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::PresentationPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::presentation_properties_part::PresentationPropertiesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::PresentationPropertiesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::query_table_part::QueryTablePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::QueryTablePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast
  for crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::QuickAccessToolbarCustomizationsPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::rd_array_part::RdArrayPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::RdArrayPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::rd_rich_value_part::RdRichValuePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::RdRichValuePart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::RdRichValueStructurePart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::rd_rich_value_types_part::RdRichValueTypesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::RdRichValueTypesPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::RdRichValueWebImagePart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast
  for crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::RdSupportingPropertyBagPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast
  for crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::RdSupportingPropertyBagStructurePart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast
  for crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::RibbonAndBackstageCustomizationsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::RibbonExtensibilityPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::rich_styles_part::RichStylesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::RichStylesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::shared_string_table_part::SharedStringTablePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::SharedStringTablePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::single_cell_table_part::SingleCellTablePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::SingleCellTablePart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::slicer_cache_part::SlicerCachePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::SlicerCachePart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::slicers_part::SlicersPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::SlicersPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::slide_comments_part::SlideCommentsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::SlideCommentsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::slide_layout_part::SlideLayoutPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::SlideLayoutPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::slide_master_part::SlideMasterPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::SlideMasterPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::slide_part::SlidePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::SlidePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::slide_sync_data_part::SlideSyncDataPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::SlideSyncDataPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast
  for crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::SpreadsheetPrinterSettingsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::style_definitions_part::StyleDefinitionsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::StyleDefinitionsPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::styles_with_effects_part::StylesWithEffectsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::StylesWithEffectsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::table_definition_part::TableDefinitionPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::TableDefinitionPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::table_styles_part::TableStylesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::TableStylesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::theme_override_part::ThemeOverridePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ThemeOverridePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::theme_part::ThemePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ThemePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::thumbnail_part::ThumbnailPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ThumbnailPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::time_line_cache_part::TimeLineCachePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::TimeLineCachePart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::time_line_part::TimeLinePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::TimeLinePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::user_defined_tags_part::UserDefinedTagsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::UserDefinedTagsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::vba_data_part::VbaDataPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::VbaDataPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::vba_project_part::VbaProjectPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::VbaProjectPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::view_properties_part::ViewPropertiesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ViewPropertiesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::vml_drawing_part::VmlDrawingPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::VmlDrawingPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::volatile_dependencies_part::VolatileDependenciesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::VolatileDependenciesPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WebExTaskpanesPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::web_extension_part::WebExtensionPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WebExtensionPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::web_settings_part::WebSettingsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WebSettingsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WordAttachedToolbarsPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WordCommentsExtensiblePart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast
  for crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WordprocessingCommentsExPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast
  for crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WordprocessingCommentsIdsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WordprocessingCommentsPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::wordprocessing_people_part::WordprocessingPeoplePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WordprocessingPeoplePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast
  for crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WordprocessingPrinterSettingsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::workbook_part::WorkbookPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WorkbookPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast for crate::parts::workbook_person_part::WorkbookPersonPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WorkbookPersonPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WorkbookRevisionHeaderPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WorkbookRevisionLogPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::workbook_styles_part::WorkbookStylesPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WorkbookStylesPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::workbook_user_data_part::WorkbookUserDataPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WorkbookUserDataPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::worksheet_comments_part::WorksheetCommentsPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WorksheetCommentsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::worksheet_part::WorksheetPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WorksheetPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::worksheet_sort_map_part::WorksheetSortMapPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WorksheetSortMapPart(part) => Some(part),
      _ => None,
    }
  }
}
#[cfg(feature = "microsoft365")]
impl PartRefDowncast
  for crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart
{
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::WorksheetThreadedCommentsPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::xml_signature_part::XmlSignaturePart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::XmlSignaturePart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRefDowncast for crate::parts::extended_part::ExtendedPart {
  #[inline]
  fn downcast_from_part_ref(part_ref: PartRef) -> Option<Self> {
    match part_ref {
      PartRef::ExtendedPart(part) => Some(part),
      _ => None,
    }
  }
}
impl PartRef {
  pub fn part_id(&self) -> crate::common::PartId {
    match self {
      PartRef::AlternativeFormatImportPart(part) => part.part_id(),
      PartRef::CalculationChainPart(part) => part.part_id(),
      PartRef::CellMetadataPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::ChartColorStylePart(part) => part.part_id(),
      PartRef::ChartDrawingPart(part) => part.part_id(),
      PartRef::ChartPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::ChartStylePart(part) => part.part_id(),
      PartRef::ChartsheetPart(part) => part.part_id(),
      PartRef::CommentAuthorsPart(part) => part.part_id(),
      PartRef::ConnectionsPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::ControlPropertiesPart(part) => part.part_id(),
      PartRef::CoreFilePropertiesPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::CustomDataPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::CustomDataPropertiesPart(part) => part.part_id(),
      PartRef::CustomFilePropertiesPart(part) => part.part_id(),
      PartRef::CustomPropertyPart(part) => part.part_id(),
      PartRef::CustomXmlMappingsPart(part) => part.part_id(),
      PartRef::CustomXmlPart(part) => part.part_id(),
      PartRef::CustomXmlPropertiesPart(part) => part.part_id(),
      PartRef::CustomizationPart(part) => part.part_id(),
      PartRef::DiagramColorsPart(part) => part.part_id(),
      PartRef::DiagramDataPart(part) => part.part_id(),
      PartRef::DiagramLayoutDefinitionPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::DiagramPersistLayoutPart(part) => part.part_id(),
      PartRef::DiagramStylePart(part) => part.part_id(),
      PartRef::DialogsheetPart(part) => part.part_id(),
      PartRef::DigitalSignatureOriginPart(part) => part.part_id(),
      PartRef::DocumentSettingsPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::DocumentTasksPart(part) => part.part_id(),
      PartRef::DrawingsPart(part) => part.part_id(),
      PartRef::EmbeddedControlPersistenceBinaryDataPart(part) => part.part_id(),
      PartRef::EmbeddedControlPersistencePart(part) => part.part_id(),
      PartRef::EmbeddedObjectPart(part) => part.part_id(),
      PartRef::EmbeddedPackagePart(part) => part.part_id(),
      PartRef::EndnotesPart(part) => part.part_id(),
      PartRef::ExcelAttachedToolbarsPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::ExtendedChartPart(part) => part.part_id(),
      PartRef::ExtendedFilePropertiesPart(part) => part.part_id(),
      PartRef::ExternalWorkbookPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::FeaturePropertyBagsPart(part) => part.part_id(),
      PartRef::FontPart(part) => part.part_id(),
      PartRef::FontTablePart(part) => part.part_id(),
      PartRef::FooterPart(part) => part.part_id(),
      PartRef::FootnotesPart(part) => part.part_id(),
      PartRef::GlossaryDocumentPart(part) => part.part_id(),
      PartRef::HandoutMasterPart(part) => part.part_id(),
      PartRef::HeaderPart(part) => part.part_id(),
      PartRef::ImagePart(part) => part.part_id(),
      PartRef::InternationalMacroSheetPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::LabelInfoPart(part) => part.part_id(),
      PartRef::LegacyDiagramTextInfoPart(part) => part.part_id(),
      PartRef::LegacyDiagramTextPart(part) => part.part_id(),
      PartRef::MacroSheetPart(part) => part.part_id(),
      PartRef::MailMergeRecipientDataPart(part) => part.part_id(),
      PartRef::MainDocumentPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::Model3DReferenceRelationshipPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::NamedSheetViewsPart(part) => part.part_id(),
      PartRef::NotesMasterPart(part) => part.part_id(),
      PartRef::NotesSlidePart(part) => part.part_id(),
      PartRef::NumberingDefinitionsPart(part) => part.part_id(),
      PartRef::PivotTableCacheDefinitionPart(part) => part.part_id(),
      PartRef::PivotTableCacheRecordsPart(part) => part.part_id(),
      PartRef::PivotTablePart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::PowerPointAuthorsPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::PowerPointCommentPart(part) => part.part_id(),
      PartRef::PresentationPart(part) => part.part_id(),
      PartRef::PresentationPropertiesPart(part) => part.part_id(),
      PartRef::QueryTablePart(part) => part.part_id(),
      PartRef::QuickAccessToolbarCustomizationsPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::RdArrayPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValuePart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValueStructurePart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValueTypesPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValueWebImagePart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::RdSupportingPropertyBagPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::RdSupportingPropertyBagStructurePart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::RibbonAndBackstageCustomizationsPart(part) => part.part_id(),
      PartRef::RibbonExtensibilityPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::RichStylesPart(part) => part.part_id(),
      PartRef::SharedStringTablePart(part) => part.part_id(),
      PartRef::SingleCellTablePart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::SlicerCachePart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::SlicersPart(part) => part.part_id(),
      PartRef::SlideCommentsPart(part) => part.part_id(),
      PartRef::SlideLayoutPart(part) => part.part_id(),
      PartRef::SlideMasterPart(part) => part.part_id(),
      PartRef::SlidePart(part) => part.part_id(),
      PartRef::SlideSyncDataPart(part) => part.part_id(),
      PartRef::SpreadsheetPrinterSettingsPart(part) => part.part_id(),
      PartRef::StyleDefinitionsPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::StylesWithEffectsPart(part) => part.part_id(),
      PartRef::TableDefinitionPart(part) => part.part_id(),
      PartRef::TableStylesPart(part) => part.part_id(),
      PartRef::ThemeOverridePart(part) => part.part_id(),
      PartRef::ThemePart(part) => part.part_id(),
      PartRef::ThumbnailPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::TimeLineCachePart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::TimeLinePart(part) => part.part_id(),
      PartRef::UserDefinedTagsPart(part) => part.part_id(),
      PartRef::VbaDataPart(part) => part.part_id(),
      PartRef::VbaProjectPart(part) => part.part_id(),
      PartRef::ViewPropertiesPart(part) => part.part_id(),
      PartRef::VmlDrawingPart(part) => part.part_id(),
      PartRef::VolatileDependenciesPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::WebExTaskpanesPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::WebExtensionPart(part) => part.part_id(),
      PartRef::WebSettingsPart(part) => part.part_id(),
      PartRef::WordAttachedToolbarsPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::WordCommentsExtensiblePart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::WordprocessingCommentsExPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::WordprocessingCommentsIdsPart(part) => part.part_id(),
      PartRef::WordprocessingCommentsPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::WordprocessingPeoplePart(part) => part.part_id(),
      PartRef::WordprocessingPrinterSettingsPart(part) => part.part_id(),
      PartRef::WorkbookPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::WorkbookPersonPart(part) => part.part_id(),
      PartRef::WorkbookRevisionHeaderPart(part) => part.part_id(),
      PartRef::WorkbookRevisionLogPart(part) => part.part_id(),
      PartRef::WorkbookStylesPart(part) => part.part_id(),
      PartRef::WorkbookUserDataPart(part) => part.part_id(),
      PartRef::WorksheetCommentsPart(part) => part.part_id(),
      PartRef::WorksheetPart(part) => part.part_id(),
      PartRef::WorksheetSortMapPart(part) => part.part_id(),
      #[cfg(feature = "microsoft365")]
      PartRef::WorksheetThreadedCommentsPart(part) => part.part_id(),
      PartRef::XmlSignaturePart(part) => part.part_id(),
      PartRef::ExtendedPart(part) => {
        <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartHandle>::part_id(part)
      }
    }
  }
  pub fn relationship_id(&self) -> Option<&str> {
    match self {
            PartRef::AlternativeFormatImportPart(part) => {
                <crate::parts::alternative_format_import_part::AlternativeFormatImportPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::CalculationChainPart(part) => {
                <crate::parts::calculation_chain_part::CalculationChainPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::CellMetadataPart(part) => {
                <crate::parts::cell_metadata_part::CellMetadataPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::ChartColorStylePart(part) => {
                <crate::parts::chart_color_style_part::ChartColorStylePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ChartDrawingPart(part) => {
                <crate::parts::chart_drawing_part::ChartDrawingPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ChartPart(part) => {
                <crate::parts::chart_part::ChartPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::ChartStylePart(part) => {
                <crate::parts::chart_style_part::ChartStylePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ChartsheetPart(part) => {
                <crate::parts::chartsheet_part::ChartsheetPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::CommentAuthorsPart(part) => {
                <crate::parts::comment_authors_part::CommentAuthorsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ConnectionsPart(part) => {
                <crate::parts::connections_part::ConnectionsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::ControlPropertiesPart(part) => {
                <crate::parts::control_properties_part::ControlPropertiesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::CoreFilePropertiesPart(part) => {
                <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::CustomDataPart(part) => {
                <crate::parts::custom_data_part::CustomDataPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::CustomDataPropertiesPart(part) => {
                <crate::parts::custom_data_properties_part::CustomDataPropertiesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::CustomFilePropertiesPart(part) => {
                <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::CustomPropertyPart(part) => {
                <crate::parts::custom_property_part::CustomPropertyPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::CustomXmlMappingsPart(part) => {
                <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::CustomXmlPart(part) => {
                <crate::parts::custom_xml_part::CustomXmlPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::CustomXmlPropertiesPart(part) => {
                <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::CustomizationPart(part) => {
                <crate::parts::customization_part::CustomizationPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::DiagramColorsPart(part) => {
                <crate::parts::diagram_colors_part::DiagramColorsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::DiagramDataPart(part) => {
                <crate::parts::diagram_data_part::DiagramDataPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::DiagramLayoutDefinitionPart(part) => {
                <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::DiagramPersistLayoutPart(part) => {
                <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::DiagramStylePart(part) => {
                <crate::parts::diagram_style_part::DiagramStylePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::DialogsheetPart(part) => {
                <crate::parts::dialogsheet_part::DialogsheetPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::DigitalSignatureOriginPart(part) => {
                <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::DocumentSettingsPart(part) => {
                <crate::parts::document_settings_part::DocumentSettingsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::DocumentTasksPart(part) => {
                <crate::parts::document_tasks_part::DocumentTasksPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::DrawingsPart(part) => {
                <crate::parts::drawings_part::DrawingsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::EmbeddedControlPersistenceBinaryDataPart(part) => {
                <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::EmbeddedControlPersistencePart(part) => {
                <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::EmbeddedObjectPart(part) => {
                <crate::parts::embedded_object_part::EmbeddedObjectPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::EmbeddedPackagePart(part) => {
                <crate::parts::embedded_package_part::EmbeddedPackagePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::EndnotesPart(part) => {
                <crate::parts::endnotes_part::EndnotesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ExcelAttachedToolbarsPart(part) => {
                <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::ExtendedChartPart(part) => {
                <crate::parts::extended_chart_part::ExtendedChartPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ExtendedFilePropertiesPart(part) => {
                <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ExternalWorkbookPart(part) => {
                <crate::parts::external_workbook_part::ExternalWorkbookPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::FeaturePropertyBagsPart(part) => {
                <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::FontPart(part) => {
                <crate::parts::font_part::FontPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::FontTablePart(part) => {
                <crate::parts::font_table_part::FontTablePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::FooterPart(part) => {
                <crate::parts::footer_part::FooterPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::FootnotesPart(part) => {
                <crate::parts::footnotes_part::FootnotesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::GlossaryDocumentPart(part) => {
                <crate::parts::glossary_document_part::GlossaryDocumentPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::HandoutMasterPart(part) => {
                <crate::parts::handout_master_part::HandoutMasterPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::HeaderPart(part) => {
                <crate::parts::header_part::HeaderPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ImagePart(part) => {
                <crate::parts::image_part::ImagePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::InternationalMacroSheetPart(part) => {
                <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::LabelInfoPart(part) => {
                <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::LegacyDiagramTextInfoPart(part) => {
                <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::LegacyDiagramTextPart(part) => {
                <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::MacroSheetPart(part) => {
                <crate::parts::macro_sheet_part::MacroSheetPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::MailMergeRecipientDataPart(part) => {
                <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::MainDocumentPart(part) => {
                <crate::parts::main_document_part::MainDocumentPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::Model3DReferenceRelationshipPart(part) => {
                <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::NamedSheetViewsPart(part) => {
                <crate::parts::named_sheet_views_part::NamedSheetViewsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::NotesMasterPart(part) => {
                <crate::parts::notes_master_part::NotesMasterPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::NotesSlidePart(part) => {
                <crate::parts::notes_slide_part::NotesSlidePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::NumberingDefinitionsPart(part) => {
                <crate::parts::numbering_definitions_part::NumberingDefinitionsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::PivotTableCacheDefinitionPart(part) => {
                <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::PivotTableCacheRecordsPart(part) => {
                <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::PivotTablePart(part) => {
                <crate::parts::pivot_table_part::PivotTablePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::PowerPointAuthorsPart(part) => {
                <crate::parts::power_point_authors_part::PowerPointAuthorsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::PowerPointCommentPart(part) => {
                <crate::parts::power_point_comment_part::PowerPointCommentPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::PresentationPart(part) => {
                <crate::parts::presentation_part::PresentationPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::PresentationPropertiesPart(part) => {
                <crate::parts::presentation_properties_part::PresentationPropertiesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::QueryTablePart(part) => {
                <crate::parts::query_table_part::QueryTablePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::QuickAccessToolbarCustomizationsPart(part) => {
                <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::RdArrayPart(part) => {
                <crate::parts::rd_array_part::RdArrayPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::RdRichValuePart(part) => {
                <crate::parts::rd_rich_value_part::RdRichValuePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::RdRichValueStructurePart(part) => {
                <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::RdRichValueTypesPart(part) => {
                <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::RdRichValueWebImagePart(part) => {
                <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::RdSupportingPropertyBagPart(part) => {
                <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::RdSupportingPropertyBagStructurePart(part) => {
                <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::RibbonAndBackstageCustomizationsPart(part) => {
                <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::RibbonExtensibilityPart(part) => {
                <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::RichStylesPart(part) => {
                <crate::parts::rich_styles_part::RichStylesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::SharedStringTablePart(part) => {
                <crate::parts::shared_string_table_part::SharedStringTablePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::SingleCellTablePart(part) => {
                <crate::parts::single_cell_table_part::SingleCellTablePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::SlicerCachePart(part) => {
                <crate::parts::slicer_cache_part::SlicerCachePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::SlicersPart(part) => {
                <crate::parts::slicers_part::SlicersPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::SlideCommentsPart(part) => {
                <crate::parts::slide_comments_part::SlideCommentsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::SlideLayoutPart(part) => {
                <crate::parts::slide_layout_part::SlideLayoutPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::SlideMasterPart(part) => {
                <crate::parts::slide_master_part::SlideMasterPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::SlidePart(part) => {
                <crate::parts::slide_part::SlidePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::SlideSyncDataPart(part) => {
                <crate::parts::slide_sync_data_part::SlideSyncDataPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::SpreadsheetPrinterSettingsPart(part) => {
                <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::StyleDefinitionsPart(part) => {
                <crate::parts::style_definitions_part::StyleDefinitionsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::StylesWithEffectsPart(part) => {
                <crate::parts::styles_with_effects_part::StylesWithEffectsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::TableDefinitionPart(part) => {
                <crate::parts::table_definition_part::TableDefinitionPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::TableStylesPart(part) => {
                <crate::parts::table_styles_part::TableStylesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ThemeOverridePart(part) => {
                <crate::parts::theme_override_part::ThemeOverridePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ThemePart(part) => {
                <crate::parts::theme_part::ThemePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ThumbnailPart(part) => {
                <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::TimeLineCachePart(part) => {
                <crate::parts::time_line_cache_part::TimeLineCachePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::TimeLinePart(part) => {
                <crate::parts::time_line_part::TimeLinePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::UserDefinedTagsPart(part) => {
                <crate::parts::user_defined_tags_part::UserDefinedTagsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::VbaDataPart(part) => {
                <crate::parts::vba_data_part::VbaDataPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::VbaProjectPart(part) => {
                <crate::parts::vba_project_part::VbaProjectPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ViewPropertiesPart(part) => {
                <crate::parts::view_properties_part::ViewPropertiesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::VmlDrawingPart(part) => {
                <crate::parts::vml_drawing_part::VmlDrawingPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::VolatileDependenciesPart(part) => {
                <crate::parts::volatile_dependencies_part::VolatileDependenciesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::WebExTaskpanesPart(part) => {
                <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::WebExtensionPart(part) => {
                <crate::parts::web_extension_part::WebExtensionPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WebSettingsPart(part) => {
                <crate::parts::web_settings_part::WebSettingsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WordAttachedToolbarsPart(part) => {
                <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::WordCommentsExtensiblePart(part) => {
                <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::WordprocessingCommentsExPart(part) => {
                <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::WordprocessingCommentsIdsPart(part) => {
                <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WordprocessingCommentsPart(part) => {
                <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::WordprocessingPeoplePart(part) => {
                <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WordprocessingPrinterSettingsPart(part) => {
                <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WorkbookPart(part) => {
                <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::WorkbookPersonPart(part) => {
                <crate::parts::workbook_person_part::WorkbookPersonPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WorkbookRevisionHeaderPart(part) => {
                <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WorkbookRevisionLogPart(part) => {
                <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WorkbookStylesPart(part) => {
                <crate::parts::workbook_styles_part::WorkbookStylesPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WorkbookUserDataPart(part) => {
                <crate::parts::workbook_user_data_part::WorkbookUserDataPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WorksheetCommentsPart(part) => {
                <crate::parts::worksheet_comments_part::WorksheetCommentsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WorksheetPart(part) => {
                <crate::parts::worksheet_part::WorksheetPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::WorksheetSortMapPart(part) => {
                <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            #[cfg(feature = "microsoft365")]
            PartRef::WorksheetThreadedCommentsPart(part) => {
                <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::XmlSignaturePart(part) => {
                <crate::parts::xml_signature_part::XmlSignaturePart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
            PartRef::ExtendedPart(part) => {
                <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartHandle>::relationship_id(
                    part,
                )
            }
        }
  }
  pub(crate) fn collect_modeled_part_relationships<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
    relationships: &mut std::collections::HashMap<
      crate::common::PartId,
      crate::common::RelationshipSet,
    >,
  ) -> Result<(), crate::common::SdkError> {
    match self {
      PartRef::AlternativeFormatImportPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::CalculationChainPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::CellMetadataPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::ChartColorStylePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ChartDrawingPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ChartPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::ChartStylePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ChartsheetPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::CommentAuthorsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ConnectionsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::ControlPropertiesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::CoreFilePropertiesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::CustomDataPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::CustomDataPropertiesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::CustomFilePropertiesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::CustomPropertyPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::CustomXmlMappingsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::CustomXmlPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::CustomXmlPropertiesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::CustomizationPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::DiagramColorsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::DiagramDataPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::DiagramLayoutDefinitionPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::DiagramPersistLayoutPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::DiagramStylePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::DialogsheetPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::DigitalSignatureOriginPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::DocumentSettingsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::DocumentTasksPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::DrawingsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::EmbeddedControlPersistenceBinaryDataPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::EmbeddedControlPersistencePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::EmbeddedObjectPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::EmbeddedPackagePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::EndnotesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ExcelAttachedToolbarsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::ExtendedChartPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ExtendedFilePropertiesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ExternalWorkbookPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::FeaturePropertyBagsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::FontPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::FontTablePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::FooterPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::FootnotesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::GlossaryDocumentPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::HandoutMasterPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::HeaderPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ImagePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::InternationalMacroSheetPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::LabelInfoPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::LegacyDiagramTextInfoPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::LegacyDiagramTextPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::MacroSheetPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::MailMergeRecipientDataPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::MainDocumentPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::Model3DReferenceRelationshipPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::NamedSheetViewsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::NotesMasterPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::NotesSlidePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::NumberingDefinitionsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::PivotTableCacheDefinitionPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::PivotTableCacheRecordsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::PivotTablePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::PowerPointAuthorsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::PowerPointCommentPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::PresentationPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::PresentationPropertiesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::QueryTablePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::QuickAccessToolbarCustomizationsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdArrayPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValuePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValueStructurePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValueTypesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdRichValueWebImagePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdSupportingPropertyBagPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RdSupportingPropertyBagStructurePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RibbonAndBackstageCustomizationsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::RibbonExtensibilityPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::RichStylesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::SharedStringTablePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::SingleCellTablePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::SlicerCachePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::SlicersPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::SlideCommentsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::SlideLayoutPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::SlideMasterPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::SlidePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::SlideSyncDataPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::SpreadsheetPrinterSettingsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::StyleDefinitionsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::StylesWithEffectsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::TableDefinitionPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::TableStylesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ThemeOverridePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ThemePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ThumbnailPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::TimeLineCachePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::TimeLinePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::UserDefinedTagsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::VbaDataPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::VbaProjectPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ViewPropertiesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::VmlDrawingPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::VolatileDependenciesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WebExTaskpanesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WebExtensionPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WebSettingsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WordAttachedToolbarsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WordCommentsExtensiblePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WordprocessingCommentsExPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WordprocessingCommentsIdsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WordprocessingCommentsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WordprocessingPeoplePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WordprocessingPrinterSettingsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WorkbookPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WorkbookPersonPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WorkbookRevisionHeaderPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WorkbookRevisionLogPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WorkbookStylesPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WorkbookUserDataPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WorksheetCommentsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WorksheetPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::WorksheetSortMapPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      #[cfg(feature = "microsoft365")]
      PartRef::WorksheetThreadedCommentsPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::XmlSignaturePart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
      PartRef::ExtendedPart(part) => {
        crate::sdk::SdkPartHandle::collect_modeled_part_relationships(part, package, relationships)
      }
    }
  }
  pub fn downcast<T: PartRefDowncast>(self) -> Option<T> {
    T::downcast_from_part_ref(self)
  }
  pub(crate) fn from_part_id<P: crate::sdk::SdkPackage>(
    package: &P,
    part_id: crate::common::PartId,
  ) -> Option<Self> {
    let part = crate::sdk::SdkPackageInternal::storage(package).part(part_id)?;
    let Some(relationship_type) = part.relationship_type() else {
      return Some(
                PartRef::ExtendedPart(
                    <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                        crate::sdk::SdkPackageInternal::storage(package),
                        part_id,
                    ),
                ),
            );
    };
    match relationship_type {
      "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary" => {
        if true {
          return Some(
                        PartRef::EmbeddedControlPersistenceBinaryDataPart(
                            <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars" => {
        if part.content_type() == "application/vnd.ms-excel.attachedToolbars" {
          return Some(
                        PartRef::ExcelAttachedToolbarsPart(
                            <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.content_type() == "application/vnd.ms-word.attachedToolbars" {
          return Some(
                        PartRef::WordAttachedToolbarsPart(
                            <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations" => {
        if part.content_type() == "application/vnd.ms-word.keyMapCustomizations+xml" {
          return Some(
                        PartRef::CustomizationPart(
                            <crate::parts::customization_part::CustomizationPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText" => {
        if part.content_type() == "application/vnd.ms-office.legacyDiagramText" {
          return Some(
                        PartRef::LegacyDiagramTextPart(
                            <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo" => {
        if part.content_type() == "application/vnd.ms-office.legacyDocTextInfo" {
          return Some(
                        PartRef::LegacyDiagramTextInfoPart(
                            <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility" => {
        if part.content_type() == "application/xml" {
          return Some(
                        PartRef::RibbonExtensibilityPart(
                            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization" => {
        if part.content_type() == "application/xml" {
          return Some(
                        PartRef::QuickAccessToolbarCustomizationsPart(
                            <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject" => {
        if part.content_type() == "application/vnd.ms-office.vbaProject" {
          return Some(
                        PartRef::VbaProjectPart(
                            <crate::parts::vba_project_part::VbaProjectPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/wordVbaData" => {
        if part.content_type() == "application/vnd.ms-word.vbaData+xml" {
          return Some(
                        PartRef::VbaDataPart(
                            <crate::parts::vba_data_part::VbaDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/wsSortMap" => {
        if part.content_type() == "application/vnd.ms-excel.wsSortMap+xml" {
          return Some(
                        PartRef::WorksheetSortMapPart(
                            <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet" => {
        if part.content_type() == "application/vnd.ms-excel.intlmacrosheet+xml" {
          return Some(
                        PartRef::InternationalMacroSheetPart(
                            <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet" => {
        if part.content_type() == "application/vnd.ms-excel.macrosheet+xml" {
          return Some(
                        PartRef::MacroSheetPart(
                            <crate::parts::macro_sheet_part::MacroSheetPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/customData" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/binary" {
          return Some(
                        PartRef::CustomDataPart(
                            <crate::parts::custom_data_part::CustomDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/customDataProps" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.customDataProperties+xml" {
          return Some(
                        PartRef::CustomDataPropertiesPart(
                            <crate::parts::custom_data_properties_part::CustomDataPropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.drawingml.diagramDrawing+xml" {
          return Some(
                        PartRef::DiagramPersistLayoutPart(
                            <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/slicer" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.slicer+xml" {
          return Some(
                        PartRef::SlicersPart(
                            <crate::parts::slicers_part::SlicersPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/slicerCache" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.slicerCache+xml" {
          return Some(
                        PartRef::SlicerCachePart(
                            <crate::parts::slicer_cache_part::SlicerCachePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-word.stylesWithEffects+xml" {
          return Some(
                        PartRef::StylesWithEffectsPart(
                            <crate::parts::styles_with_effects_part::StylesWithEffectsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/xml" {
          return Some(
                        PartRef::RibbonAndBackstageCustomizationsPart(
                            <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.chartcolorstyle+xml" {
          return Some(
                        PartRef::ChartColorStylePart(
                            <crate::parts::chart_color_style_part::ChartColorStylePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/chartStyle" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.chartstyle+xml" {
          return Some(
                        PartRef::ChartStylePart(
                            <crate::parts::chart_style_part::ChartStylePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/commentsExtended" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml"
        {
          return Some(
                        PartRef::WordprocessingCommentsExPart(
                            <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/people" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml"
        {
          return Some(
                        PartRef::WordprocessingPeoplePart(
                            <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/timeline" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.timeline+xml" {
          return Some(
                        PartRef::TimeLinePart(
                            <crate::parts::time_line_part::TimeLinePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/timelineCache" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.timelineCache+xml" {
          return Some(
                        PartRef::TimeLineCachePart(
                            <crate::parts::time_line_cache_part::TimeLineCachePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/webextension" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.webextension+xml" {
          return Some(
                        PartRef::WebExtensionPart(
                            <crate::parts::web_extension_part::WebExtensionPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.webextensiontaskpanes+xml" {
          return Some(
                        PartRef::WebExTaskpanesPart(
                            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2014/relationships/chartEx" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.chartex+xml" {
          return Some(
                        PartRef::ExtendedChartPart(
                            <crate::parts::extended_chart_part::ExtendedChartPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml"
        {
          return Some(
                        PartRef::WordprocessingCommentsIdsPart(
                            <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "model/gltf-binary" {
          return Some(
                        PartRef::Model3DReferenceRelationshipPart(
                            <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdArray" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdarray+xml" {
          return Some(
                        PartRef::RdArrayPart(
                            <crate::parts::rd_array_part::RdArrayPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvalue+xml" {
          return Some(
                        PartRef::RdRichValuePart(
                            <crate::parts::rd_rich_value_part::RdRichValuePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluestructure+xml" {
          return Some(
                        PartRef::RdRichValueStructurePart(
                            <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluetypes+xml" {
          return Some(
                        PartRef::RdRichValueTypesPart(
                            <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdsupportingpropertybag+xml" {
          return Some(
                        PartRef::RdSupportingPropertyBagPart(
                            <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure" =>
      {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml" {
          return Some(
                        PartRef::RdSupportingPropertyBagStructurePart(
                            <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/richStyles" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.richstyles+xml" {
          return Some(
                        PartRef::RichStylesPart(
                            <crate::parts::rich_styles_part::RichStylesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/10/relationships/person" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.person+xml" {
          return Some(
                        PartRef::WorkbookPersonPart(
                            <crate::parts::workbook_person_part::WorkbookPersonPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.threadedcomments+xml" {
          return Some(
                        PartRef::WorksheetThreadedCommentsPart(
                            <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml"
        {
          return Some(
                        PartRef::WordCommentsExtensiblePart(
                            <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2018/10/relationships/authors" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-powerpoint.authors+xml" {
          return Some(
                        PartRef::PowerPointAuthorsPart(
                            <crate::parts::power_point_authors_part::PowerPointAuthorsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2018/10/relationships/comments" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-powerpoint.comments+xml" {
          return Some(
                        PartRef::PowerPointCommentPart(
                            <crate::parts::power_point_comment_part::PowerPointCommentPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.namedsheetviews+xml" {
          return Some(
                        PartRef::NamedSheetViewsPart(
                            <crate::parts::named_sheet_views_part::NamedSheetViewsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.documenttasks+xml" {
          return Some(
                        PartRef::DocumentTasksPart(
                            <crate::parts::document_tasks_part::DocumentTasksPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.classificationlabels+xml" {
          return Some(
                        PartRef::LabelInfoPart(
                            <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluewebimage+xml" {
          return Some(
                        PartRef::RdRichValueWebImagePart(
                            <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.featurepropertybag+xml" {
          return Some(
                        PartRef::FeaturePropertyBagsPart(
                            <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk" => {
        if true {
          return Some(
                        PartRef::AlternativeFormatImportPart(
                            <crate::parts::alternative_format_import_part::AlternativeFormatImportPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"
        {
          return Some(
                        PartRef::CalculationChainPart(
                            <crate::parts::calculation_chain_part::CalculationChainPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"
        {
          return Some(
                        PartRef::ChartPart(
                            <crate::parts::chart_part::ChartPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"
        {
          return Some(
                        PartRef::ChartDrawingPart(
                            <crate::parts::chart_drawing_part::ChartDrawingPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"
        {
          return Some(
                        PartRef::ChartsheetPart(
                            <crate::parts::chartsheet_part::ChartsheetPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"
        {
          return Some(
                        PartRef::CommentAuthorsPart(
                            <crate::parts::comment_authors_part::CommentAuthorsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.comments+xml"
        {
          return Some(
                        PartRef::SlideCommentsPart(
                            <crate::parts::slide_comments_part::SlideCommentsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"
        {
          return Some(
                        PartRef::WordprocessingCommentsPart(
                            <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"
        {
          return Some(
                        PartRef::WorksheetCommentsPart(
                            <crate::parts::worksheet_comments_part::WorksheetCommentsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"
        {
          return Some(
                        PartRef::ConnectionsPart(
                            <crate::parts::connections_part::ConnectionsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control" => {
        if true {
          return Some(
                        PartRef::EmbeddedControlPersistencePart(
                            <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.controlproperties+xml" {
          return Some(
                        PartRef::ControlPropertiesPart(
                            <crate::parts::control_properties_part::ControlPropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.custom-properties+xml"
        {
          return Some(
                        PartRef::CustomFilePropertiesPart(
                            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty" => {
        if true {
          return Some(
                        PartRef::CustomPropertyPart(
                            <crate::parts::custom_property_part::CustomPropertyPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml" => {
        if true {
          return Some(
                        PartRef::CustomXmlPart(
                            <crate::parts::custom_xml_part::CustomXmlPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.customXmlProperties+xml"
        {
          return Some(
                        PartRef::CustomXmlPropertiesPart(
                            <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"
        {
          return Some(
                        PartRef::DiagramColorsPart(
                            <crate::parts::diagram_colors_part::DiagramColorsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"
        {
          return Some(
                        PartRef::DiagramDataPart(
                            <crate::parts::diagram_data_part::DiagramDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"
        {
          return Some(
                        PartRef::DiagramLayoutDefinitionPart(
                            <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"
        {
          return Some(
                        PartRef::DiagramStylePart(
                            <crate::parts::diagram_style_part::DiagramStylePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"
        {
          return Some(
                        PartRef::DialogsheetPart(
                            <crate::parts::dialogsheet_part::DialogsheetPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.drawing+xml" {
          return Some(
                        PartRef::DrawingsPart(
                            <crate::parts::drawings_part::DrawingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"
        {
          return Some(
                        PartRef::EndnotesPart(
                            <crate::parts::endnotes_part::EndnotesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.extended-properties+xml"
        {
          return Some(
                        PartRef::ExtendedFilePropertiesPart(
                            <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"
        {
          return Some(
                        PartRef::ExternalWorkbookPart(
                            <crate::parts::external_workbook_part::ExternalWorkbookPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font" => {
        if true {
          return Some(
                        PartRef::FontPart(
                            <crate::parts::font_part::FontPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"
        {
          return Some(
                        PartRef::FontTablePart(
                            <crate::parts::font_table_part::FontTablePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"
        {
          return Some(
                        PartRef::FooterPart(
                            <crate::parts::footer_part::FooterPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"
        {
          return Some(
                        PartRef::FootnotesPart(
                            <crate::parts::footnotes_part::FootnotesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"
        {
          return Some(
                        PartRef::GlossaryDocumentPart(
                            <crate::parts::glossary_document_part::GlossaryDocumentPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"
        {
          return Some(
                        PartRef::HandoutMasterPart(
                            <crate::parts::handout_master_part::HandoutMasterPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"
        {
          return Some(
                        PartRef::HeaderPart(
                            <crate::parts::header_part::HeaderPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" => {
        if true {
          return Some(
                        PartRef::ImagePart(
                            <crate::parts::image_part::ImagePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"
        {
          return Some(
                        PartRef::NotesMasterPart(
                            <crate::parts::notes_master_part::NotesMasterPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"
        {
          return Some(
                        PartRef::NotesSlidePart(
                            <crate::parts::notes_slide_part::NotesSlidePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"
        {
          return Some(
                        PartRef::NumberingDefinitionsPart(
                            <crate::parts::numbering_definitions_part::NumberingDefinitionsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" => {
        if part.path() == "word/document.xml" {
          return Some(
                        PartRef::MainDocumentPart(
                            <crate::parts::main_document_part::MainDocumentPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.path() == "ppt/presentation.xml" {
          return Some(
                        PartRef::PresentationPart(
                            <crate::parts::presentation_part::PresentationPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.path() == "xl/workbook.xml" {
          return Some(
                        PartRef::WorkbookPart(
                            <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject" => {
        if true {
          return Some(
                        PartRef::EmbeddedObjectPart(
                            <crate::parts::embedded_object_part::EmbeddedObjectPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package" => {
        if true {
          return Some(
                        PartRef::EmbeddedPackagePart(
                            <crate::parts::embedded_package_part::EmbeddedPackagePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"
        {
          return Some(
                        PartRef::PivotTableCacheDefinitionPart(
                            <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"
        {
          return Some(
                        PartRef::PivotTableCacheRecordsPart(
                            <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"
        {
          return Some(
                        PartRef::PivotTablePart(
                            <crate::parts::pivot_table_part::PivotTablePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"
        {
          return Some(
                        PartRef::PresentationPropertiesPart(
                            <crate::parts::presentation_properties_part::PresentationPropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings"
        {
          return Some(
                        PartRef::SpreadsheetPrinterSettingsPart(
                            <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.printerSettings"
        {
          return Some(
                        PartRef::WordprocessingPrinterSettingsPart(
                            <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"
        {
          return Some(
                        PartRef::QueryTablePart(
                            <crate::parts::query_table_part::QueryTablePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData" => {
        if true {
          return Some(
                        PartRef::MailMergeRecipientDataPart(
                            <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"
        {
          return Some(
                        PartRef::WorkbookRevisionHeaderPart(
                            <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"
        {
          return Some(
                        PartRef::WorkbookRevisionLogPart(
                            <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"
        {
          return Some(
                        PartRef::DocumentSettingsPart(
                            <crate::parts::document_settings_part::DocumentSettingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"
        {
          return Some(
                        PartRef::SharedStringTablePart(
                            <crate::parts::shared_string_table_part::SharedStringTablePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"
        {
          return Some(
                        PartRef::CellMetadataPart(
                            <crate::parts::cell_metadata_part::CellMetadataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
        {
          return Some(
                        PartRef::SlidePart(
                            <crate::parts::slide_part::SlidePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"
        {
          return Some(
                        PartRef::SlideLayoutPart(
                            <crate::parts::slide_layout_part::SlideLayoutPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"
        {
          return Some(
                        PartRef::SlideMasterPart(
                            <crate::parts::slide_master_part::SlideMasterPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"
        {
          return Some(
                        PartRef::SlideSyncDataPart(
                            <crate::parts::slide_sync_data_part::SlideSyncDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"
        {
          return Some(
                        PartRef::StyleDefinitionsPart(
                            <crate::parts::style_definitions_part::StyleDefinitionsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"
        {
          return Some(
                        PartRef::WorkbookStylesPart(
                            <crate::parts::workbook_styles_part::WorkbookStylesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"
        {
          return Some(
                        PartRef::TableDefinitionPart(
                            <crate::parts::table_definition_part::TableDefinitionPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"
        {
          return Some(
                        PartRef::SingleCellTablePart(
                            <crate::parts::single_cell_table_part::SingleCellTablePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"
        {
          return Some(
                        PartRef::TableStylesPart(
                            <crate::parts::table_styles_part::TableStylesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.tags+xml"
        {
          return Some(
                        PartRef::UserDefinedTagsPart(
                            <crate::parts::user_defined_tags_part::UserDefinedTagsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.theme+xml" {
          return Some(
                        PartRef::ThemePart(
                            <crate::parts::theme_part::ThemePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.themeOverride+xml"
        {
          return Some(
                        PartRef::ThemeOverridePart(
                            <crate::parts::theme_override_part::ThemeOverridePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"
        {
          return Some(
                        PartRef::WorkbookUserDataPart(
                            <crate::parts::workbook_user_data_part::WorkbookUserDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"
        {
          return Some(
                        PartRef::ViewPropertiesPart(
                            <crate::parts::view_properties_part::ViewPropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.vmlDrawing" {
          return Some(
                        PartRef::VmlDrawingPart(
                            <crate::parts::vml_drawing_part::VmlDrawingPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"
        {
          return Some(
                        PartRef::VolatileDependenciesPart(
                            <crate::parts::volatile_dependencies_part::VolatileDependenciesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"
        {
          return Some(
                        PartRef::WebSettingsPart(
                            <crate::parts::web_settings_part::WebSettingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"
        {
          return Some(
                        PartRef::WorksheetPart(
                            <crate::parts::worksheet_part::WorksheetPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps" => {
        if part.content_type() == "application/xml" {
          return Some(
                        PartRef::CustomXmlMappingsPart(
                            <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin" => {
        if part.content_type() == "application/vnd.openxmlformats-package.digital-signature-origin"
        {
          return Some(
                        PartRef::DigitalSignatureOriginPart(
                            <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature" => {
        if part.content_type()
          == "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml"
        {
          return Some(
                        PartRef::XmlSignaturePart(
                            <crate::parts::xml_signature_part::XmlSignaturePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" => {
        if part.content_type() == "application/vnd.openxmlformats-package.core-properties+xml" {
          return Some(
                        PartRef::CoreFilePropertiesPart(
                            <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail" => {
        if true {
          return Some(
                        PartRef::ThumbnailPart(
                            <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::AlternativeFormatImportPart(
                            <crate::parts::alternative_format_import_part::AlternativeFormatImportPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::CalculationChainPart(
                            <crate::parts::calculation_chain_part::CalculationChainPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::CellMetadataPart(
                            <crate::parts::cell_metadata_part::CellMetadataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::ChartColorStylePart(
                            <crate::parts::chart_color_style_part::ChartColorStylePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::ChartDrawingPart(
                            <crate::parts::chart_drawing_part::ChartDrawingPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
          return Some(
                        PartRef::ChartPart(
                            <crate::parts::chart_part::ChartPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::ChartStylePart(
                            <crate::parts::chart_style_part::ChartStylePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::ChartsheetPart(
                            <crate::parts::chartsheet_part::ChartsheetPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::CommentAuthorsPart(
                            <crate::parts::comment_authors_part::CommentAuthorsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::ConnectionsPart(
                            <crate::parts::connections_part::ConnectionsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::ControlPropertiesPart(
                            <crate::parts::control_properties_part::ControlPropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::CoreFilePropertiesPart(
                            <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::CustomDataPart(
                            <crate::parts::custom_data_part::CustomDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::CustomDataPropertiesPart(
                            <crate::parts::custom_data_properties_part::CustomDataPropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::CustomFilePropertiesPart(
                            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::CustomPropertyPart(
                            <crate::parts::custom_property_part::CustomPropertyPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::CustomXmlMappingsPart(
                            <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::CustomXmlPart(
                            <crate::parts::custom_xml_part::CustomXmlPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::CustomXmlPropertiesPart(
                            <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::CustomizationPart(
                            <crate::parts::customization_part::CustomizationPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::DiagramColorsPart(
                            <crate::parts::diagram_colors_part::DiagramColorsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::DiagramDataPart(
                            <crate::parts::diagram_data_part::DiagramDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::DiagramLayoutDefinitionPart(
                            <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::DiagramPersistLayoutPart(
                            <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::DiagramStylePart(
                            <crate::parts::diagram_style_part::DiagramStylePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::DialogsheetPart(
                            <crate::parts::dialogsheet_part::DialogsheetPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::DigitalSignatureOriginPart(
                            <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::DocumentSettingsPart(
                            <crate::parts::document_settings_part::DocumentSettingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::DocumentTasksPart(
                            <crate::parts::document_tasks_part::DocumentTasksPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
          return Some(
                        PartRef::DrawingsPart(
                            <crate::parts::drawings_part::DrawingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::EmbeddedControlPersistenceBinaryDataPart(
                            <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::EmbeddedControlPersistencePart(
                            <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::EmbeddedObjectPart(
                            <crate::parts::embedded_object_part::EmbeddedObjectPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::EmbeddedPackagePart(
                            <crate::parts::embedded_package_part::EmbeddedPackagePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
          return Some(
                        PartRef::EndnotesPart(
                            <crate::parts::endnotes_part::EndnotesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::ExcelAttachedToolbarsPart(
                            <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::ExtendedChartPart(
                            <crate::parts::extended_chart_part::ExtendedChartPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::ExtendedFilePropertiesPart(
                            <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::ExternalWorkbookPart(
                            <crate::parts::external_workbook_part::ExternalWorkbookPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::FeaturePropertyBagsPart(
                            <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
          return Some(
                        PartRef::FontPart(
                            <crate::parts::font_part::FontPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::FontTablePart(
                            <crate::parts::font_table_part::FontTablePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
          return Some(
                        PartRef::FooterPart(
                            <crate::parts::footer_part::FooterPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::FootnotesPart(
                            <crate::parts::footnotes_part::FootnotesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::GlossaryDocumentPart(
                            <crate::parts::glossary_document_part::GlossaryDocumentPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::HandoutMasterPart(
                            <crate::parts::handout_master_part::HandoutMasterPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
          return Some(
                        PartRef::HeaderPart(
                            <crate::parts::header_part::HeaderPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::ImagePart(
                            <crate::parts::image_part::ImagePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::InternationalMacroSheetPart(
                            <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::LabelInfoPart(
                            <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::LegacyDiagramTextInfoPart(
                            <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::LegacyDiagramTextPart(
                            <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::MacroSheetPart(
                            <crate::parts::macro_sheet_part::MacroSheetPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::MailMergeRecipientDataPart(
                            <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::MainDocumentPart(
                            <crate::parts::main_document_part::MainDocumentPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::Model3DReferenceRelationshipPart(
                            <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::NamedSheetViewsPart(
                            <crate::parts::named_sheet_views_part::NamedSheetViewsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::NotesMasterPart(
                            <crate::parts::notes_master_part::NotesMasterPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::NotesSlidePart(
                            <crate::parts::notes_slide_part::NotesSlidePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::NumberingDefinitionsPart(
                            <crate::parts::numbering_definitions_part::NumberingDefinitionsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::PivotTableCacheDefinitionPart(
                            <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::PivotTableCacheRecordsPart(
                            <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::PivotTablePart(
                            <crate::parts::pivot_table_part::PivotTablePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::PowerPointAuthorsPart(
                            <crate::parts::power_point_authors_part::PowerPointAuthorsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::PowerPointCommentPart(
                            <crate::parts::power_point_comment_part::PowerPointCommentPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::PresentationPart(
                            <crate::parts::presentation_part::PresentationPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::PresentationPropertiesPart(
                            <crate::parts::presentation_properties_part::PresentationPropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::QueryTablePart(
                            <crate::parts::query_table_part::QueryTablePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::QuickAccessToolbarCustomizationsPart(
                            <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdArrayPart(
                            <crate::parts::rd_array_part::RdArrayPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdRichValuePart(
                            <crate::parts::rd_rich_value_part::RdRichValuePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdRichValueStructurePart(
                            <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdRichValueTypesPart(
                            <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdRichValueWebImagePart(
                            <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdSupportingPropertyBagPart(
                            <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdSupportingPropertyBagStructurePart(
                            <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RibbonAndBackstageCustomizationsPart(
                            <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::RibbonExtensibilityPart(
                            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RichStylesPart(
                            <crate::parts::rich_styles_part::RichStylesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::SharedStringTablePart(
                            <crate::parts::shared_string_table_part::SharedStringTablePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::SingleCellTablePart(
                            <crate::parts::single_cell_table_part::SingleCellTablePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::SlicerCachePart(
                            <crate::parts::slicer_cache_part::SlicerCachePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::SlicersPart(
                            <crate::parts::slicers_part::SlicersPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::SlideCommentsPart(
                            <crate::parts::slide_comments_part::SlideCommentsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::SlideLayoutPart(
                            <crate::parts::slide_layout_part::SlideLayoutPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::SlideMasterPart(
                            <crate::parts::slide_master_part::SlideMasterPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
          return Some(
                        PartRef::SlidePart(
                            <crate::parts::slide_part::SlidePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::SlideSyncDataPart(
                            <crate::parts::slide_sync_data_part::SlideSyncDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::SpreadsheetPrinterSettingsPart(
                            <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::StyleDefinitionsPart(
                            <crate::parts::style_definitions_part::StyleDefinitionsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::StylesWithEffectsPart(
                            <crate::parts::styles_with_effects_part::StylesWithEffectsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::TableDefinitionPart(
                            <crate::parts::table_definition_part::TableDefinitionPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::TableStylesPart(
                            <crate::parts::table_styles_part::TableStylesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::ThemeOverridePart(
                            <crate::parts::theme_override_part::ThemeOverridePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
          return Some(
                        PartRef::ThemePart(
                            <crate::parts::theme_part::ThemePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::ThumbnailPart(
                            <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::TimeLineCachePart(
                            <crate::parts::time_line_cache_part::TimeLineCachePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::TimeLinePart(
                            <crate::parts::time_line_part::TimeLinePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::UserDefinedTagsPart(
                            <crate::parts::user_defined_tags_part::UserDefinedTagsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
          return Some(
                        PartRef::VbaDataPart(
                            <crate::parts::vba_data_part::VbaDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::VbaProjectPart(
                            <crate::parts::vba_project_part::VbaProjectPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::ViewPropertiesPart(
                            <crate::parts::view_properties_part::ViewPropertiesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::VmlDrawingPart(
                            <crate::parts::vml_drawing_part::VmlDrawingPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::VolatileDependenciesPart(
                            <crate::parts::volatile_dependencies_part::VolatileDependenciesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WebExTaskpanesPart(
                            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WebExtensionPart(
                            <crate::parts::web_extension_part::WebExtensionPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WebSettingsPart(
                            <crate::parts::web_settings_part::WebSettingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WordAttachedToolbarsPart(
                            <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WordCommentsExtensiblePart(
                            <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WordprocessingCommentsExPart(
                            <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WordprocessingCommentsIdsPart(
                            <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WordprocessingCommentsPart(
                            <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WordprocessingPeoplePart(
                            <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WordprocessingPrinterSettingsPart(
                            <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
          return Some(
                        PartRef::WorkbookPart(
                            <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WorkbookPersonPart(
                            <crate::parts::workbook_person_part::WorkbookPersonPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WorkbookRevisionHeaderPart(
                            <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WorkbookRevisionLogPart(
                            <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WorkbookStylesPart(
                            <crate::parts::workbook_styles_part::WorkbookStylesPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WorkbookUserDataPart(
                            <crate::parts::workbook_user_data_part::WorkbookUserDataPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WorksheetCommentsPart(
                            <crate::parts::worksheet_comments_part::WorksheetCommentsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WorksheetPart(
                            <crate::parts::worksheet_part::WorksheetPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::WorksheetSortMapPart(
                            <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WorksheetThreadedCommentsPart(
                            <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
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
                        PartRef::XmlSignaturePart(
                            <crate::parts::xml_signature_part::XmlSignaturePart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                                crate::sdk::SdkPackageInternal::storage(package),
                                part_id,
                            ),
                        ),
                    );
        }
      }
    }
    Some(
            PartRef::ExtendedPart(
                <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartHandle>::from_part_id_with_relationships(
                    crate::sdk::SdkPackageInternal::storage(package),
                    part_id,
                ),
            ),
        )
  }
  pub(crate) fn from_relationship_storage(
    storage: &crate::common::SdkPackageStorage,
    relationship: &crate::common::RelationshipInfo,
  ) -> Option<Self> {
    let part_id = relationship.target_part_id()?;
    let part = storage.part(part_id)?;
    let Some(relationship_type) = part.relationship_type() else {
      return Some(
                PartRef::ExtendedPart(
                    <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                        storage,
                        relationship.id(),
                        part_id,
                    ),
                ),
            );
    };
    match relationship_type {
      "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary" => {
        if true {
          return Some(
                        PartRef::EmbeddedControlPersistenceBinaryDataPart(
                            <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars" => {
        if part.content_type() == "application/vnd.ms-excel.attachedToolbars" {
          return Some(
                        PartRef::ExcelAttachedToolbarsPart(
                            <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.content_type() == "application/vnd.ms-word.attachedToolbars" {
          return Some(
                        PartRef::WordAttachedToolbarsPart(
                            <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations" => {
        if part.content_type() == "application/vnd.ms-word.keyMapCustomizations+xml" {
          return Some(
                        PartRef::CustomizationPart(
                            <crate::parts::customization_part::CustomizationPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText" => {
        if part.content_type() == "application/vnd.ms-office.legacyDiagramText" {
          return Some(
                        PartRef::LegacyDiagramTextPart(
                            <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo" => {
        if part.content_type() == "application/vnd.ms-office.legacyDocTextInfo" {
          return Some(
                        PartRef::LegacyDiagramTextInfoPart(
                            <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility" => {
        if part.content_type() == "application/xml" {
          return Some(
                        PartRef::RibbonExtensibilityPart(
                            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization" => {
        if part.content_type() == "application/xml" {
          return Some(
                        PartRef::QuickAccessToolbarCustomizationsPart(
                            <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject" => {
        if part.content_type() == "application/vnd.ms-office.vbaProject" {
          return Some(
                        PartRef::VbaProjectPart(
                            <crate::parts::vba_project_part::VbaProjectPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/wordVbaData" => {
        if part.content_type() == "application/vnd.ms-word.vbaData+xml" {
          return Some(
                        PartRef::VbaDataPart(
                            <crate::parts::vba_data_part::VbaDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/wsSortMap" => {
        if part.content_type() == "application/vnd.ms-excel.wsSortMap+xml" {
          return Some(
                        PartRef::WorksheetSortMapPart(
                            <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/xlIntlMacrosheet" => {
        if part.content_type() == "application/vnd.ms-excel.intlmacrosheet+xml" {
          return Some(
                        PartRef::InternationalMacroSheetPart(
                            <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2006/relationships/xlMacrosheet" => {
        if part.content_type() == "application/vnd.ms-excel.macrosheet+xml" {
          return Some(
                        PartRef::MacroSheetPart(
                            <crate::parts::macro_sheet_part::MacroSheetPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/customData" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/binary" {
          return Some(
                        PartRef::CustomDataPart(
                            <crate::parts::custom_data_part::CustomDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/customDataProps" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.customDataProperties+xml" {
          return Some(
                        PartRef::CustomDataPropertiesPart(
                            <crate::parts::custom_data_properties_part::CustomDataPropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.drawingml.diagramDrawing+xml" {
          return Some(
                        PartRef::DiagramPersistLayoutPart(
                            <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/slicer" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.slicer+xml" {
          return Some(
                        PartRef::SlicersPart(
                            <crate::parts::slicers_part::SlicersPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/slicerCache" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.slicerCache+xml" {
          return Some(
                        PartRef::SlicerCachePart(
                            <crate::parts::slicer_cache_part::SlicerCachePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-word.stylesWithEffects+xml" {
          return Some(
                        PartRef::StylesWithEffectsPart(
                            <crate::parts::styles_with_effects_part::StylesWithEffectsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/xml" {
          return Some(
                        PartRef::RibbonAndBackstageCustomizationsPart(
                            <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.chartcolorstyle+xml" {
          return Some(
                        PartRef::ChartColorStylePart(
                            <crate::parts::chart_color_style_part::ChartColorStylePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/chartStyle" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.chartstyle+xml" {
          return Some(
                        PartRef::ChartStylePart(
                            <crate::parts::chart_style_part::ChartStylePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/commentsExtended" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml"
        {
          return Some(
                        PartRef::WordprocessingCommentsExPart(
                            <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/people" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml"
        {
          return Some(
                        PartRef::WordprocessingPeoplePart(
                            <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/timeline" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.timeline+xml" {
          return Some(
                        PartRef::TimeLinePart(
                            <crate::parts::time_line_part::TimeLinePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/timelineCache" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.timelineCache+xml" {
          return Some(
                        PartRef::TimeLineCachePart(
                            <crate::parts::time_line_cache_part::TimeLineCachePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/webextension" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.webextension+xml" {
          return Some(
                        PartRef::WebExtensionPart(
                            <crate::parts::web_extension_part::WebExtensionPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.webextensiontaskpanes+xml" {
          return Some(
                        PartRef::WebExTaskpanesPart(
                            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2014/relationships/chartEx" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.chartex+xml" {
          return Some(
                        PartRef::ExtendedChartPart(
                            <crate::parts::extended_chart_part::ExtendedChartPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml"
        {
          return Some(
                        PartRef::WordprocessingCommentsIdsPart(
                            <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "model/gltf-binary" {
          return Some(
                        PartRef::Model3DReferenceRelationshipPart(
                            <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdArray" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdarray+xml" {
          return Some(
                        PartRef::RdArrayPart(
                            <crate::parts::rd_array_part::RdArrayPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValue" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvalue+xml" {
          return Some(
                        PartRef::RdRichValuePart(
                            <crate::parts::rd_rich_value_part::RdRichValuePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluestructure+xml" {
          return Some(
                        PartRef::RdRichValueStructurePart(
                            <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluetypes+xml" {
          return Some(
                        PartRef::RdRichValueTypesPart(
                            <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdsupportingpropertybag+xml" {
          return Some(
                        PartRef::RdSupportingPropertyBagPart(
                            <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure" =>
      {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml" {
          return Some(
                        PartRef::RdSupportingPropertyBagStructurePart(
                            <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/06/relationships/richStyles" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.richstyles+xml" {
          return Some(
                        PartRef::RichStylesPart(
                            <crate::parts::rich_styles_part::RichStylesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/10/relationships/person" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.person+xml" {
          return Some(
                        PartRef::WorkbookPersonPart(
                            <crate::parts::workbook_person_part::WorkbookPersonPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2017/10/relationships/threadedComment" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.threadedcomments+xml" {
          return Some(
                        PartRef::WorksheetThreadedCommentsPart(
                            <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml"
        {
          return Some(
                        PartRef::WordCommentsExtensiblePart(
                            <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2018/10/relationships/authors" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-powerpoint.authors+xml" {
          return Some(
                        PartRef::PowerPointAuthorsPart(
                            <crate::parts::power_point_authors_part::PowerPointAuthorsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2018/10/relationships/comments" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-powerpoint.comments+xml" {
          return Some(
                        PartRef::PowerPointCommentPart(
                            <crate::parts::power_point_comment_part::PowerPointCommentPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2019/04/relationships/namedSheetView" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.namedsheetviews+xml" {
          return Some(
                        PartRef::NamedSheetViewsPart(
                            <crate::parts::named_sheet_views_part::NamedSheetViewsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.documenttasks+xml" {
          return Some(
                        PartRef::DocumentTasksPart(
                            <crate::parts::document_tasks_part::DocumentTasksPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-office.classificationlabels+xml" {
          return Some(
                        PartRef::LabelInfoPart(
                            <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2020/07/relationships/rdRichValueWebImage" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.rdrichvaluewebimage+xml" {
          return Some(
                        PartRef::RdRichValueWebImagePart(
                            <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.featurepropertybag+xml" {
          return Some(
                        PartRef::FeaturePropertyBagsPart(
                            <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk" => {
        if true {
          return Some(
                        PartRef::AlternativeFormatImportPart(
                            <crate::parts::alternative_format_import_part::AlternativeFormatImportPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/calcChain" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"
        {
          return Some(
                        PartRef::CalculationChainPart(
                            <crate::parts::calculation_chain_part::CalculationChainPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"
        {
          return Some(
                        PartRef::ChartPart(
                            <crate::parts::chart_part::ChartPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"
        {
          return Some(
                        PartRef::ChartDrawingPart(
                            <crate::parts::chart_drawing_part::ChartDrawingPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartsheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"
        {
          return Some(
                        PartRef::ChartsheetPart(
                            <crate::parts::chartsheet_part::ChartsheetPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"
        {
          return Some(
                        PartRef::CommentAuthorsPart(
                            <crate::parts::comment_authors_part::CommentAuthorsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.comments+xml"
        {
          return Some(
                        PartRef::SlideCommentsPart(
                            <crate::parts::slide_comments_part::SlideCommentsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"
        {
          return Some(
                        PartRef::WordprocessingCommentsPart(
                            <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"
        {
          return Some(
                        PartRef::WorksheetCommentsPart(
                            <crate::parts::worksheet_comments_part::WorksheetCommentsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/connections" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"
        {
          return Some(
                        PartRef::ConnectionsPart(
                            <crate::parts::connections_part::ConnectionsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control" => {
        if true {
          return Some(
                        PartRef::EmbeddedControlPersistencePart(
                            <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp" => {
        #[cfg(feature = "microsoft365")]
        if part.content_type() == "application/vnd.ms-excel.controlproperties+xml" {
          return Some(
                        PartRef::ControlPropertiesPart(
                            <crate::parts::control_properties_part::ControlPropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.custom-properties+xml"
        {
          return Some(
                        PartRef::CustomFilePropertiesPart(
                            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customProperty" => {
        if true {
          return Some(
                        PartRef::CustomPropertyPart(
                            <crate::parts::custom_property_part::CustomPropertyPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml" => {
        if true {
          return Some(
                        PartRef::CustomXmlPart(
                            <crate::parts::custom_xml_part::CustomXmlPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.customXmlProperties+xml"
        {
          return Some(
                        PartRef::CustomXmlPropertiesPart(
                            <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"
        {
          return Some(
                        PartRef::DiagramColorsPart(
                            <crate::parts::diagram_colors_part::DiagramColorsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"
        {
          return Some(
                        PartRef::DiagramDataPart(
                            <crate::parts::diagram_data_part::DiagramDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"
        {
          return Some(
                        PartRef::DiagramLayoutDefinitionPart(
                            <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"
        {
          return Some(
                        PartRef::DiagramStylePart(
                            <crate::parts::diagram_style_part::DiagramStylePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/dialogsheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"
        {
          return Some(
                        PartRef::DialogsheetPart(
                            <crate::parts::dialogsheet_part::DialogsheetPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.drawing+xml" {
          return Some(
                        PartRef::DrawingsPart(
                            <crate::parts::drawings_part::DrawingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"
        {
          return Some(
                        PartRef::EndnotesPart(
                            <crate::parts::endnotes_part::EndnotesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.extended-properties+xml"
        {
          return Some(
                        PartRef::ExtendedFilePropertiesPart(
                            <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"
        {
          return Some(
                        PartRef::ExternalWorkbookPart(
                            <crate::parts::external_workbook_part::ExternalWorkbookPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font" => {
        if true {
          return Some(
                        PartRef::FontPart(
                            <crate::parts::font_part::FontPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"
        {
          return Some(
                        PartRef::FontTablePart(
                            <crate::parts::font_table_part::FontTablePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"
        {
          return Some(
                        PartRef::FooterPart(
                            <crate::parts::footer_part::FooterPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"
        {
          return Some(
                        PartRef::FootnotesPart(
                            <crate::parts::footnotes_part::FootnotesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"
        {
          return Some(
                        PartRef::GlossaryDocumentPart(
                            <crate::parts::glossary_document_part::GlossaryDocumentPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"
        {
          return Some(
                        PartRef::HandoutMasterPart(
                            <crate::parts::handout_master_part::HandoutMasterPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"
        {
          return Some(
                        PartRef::HeaderPart(
                            <crate::parts::header_part::HeaderPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" => {
        if true {
          return Some(
                        PartRef::ImagePart(
                            <crate::parts::image_part::ImagePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"
        {
          return Some(
                        PartRef::NotesMasterPart(
                            <crate::parts::notes_master_part::NotesMasterPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"
        {
          return Some(
                        PartRef::NotesSlidePart(
                            <crate::parts::notes_slide_part::NotesSlidePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"
        {
          return Some(
                        PartRef::NumberingDefinitionsPart(
                            <crate::parts::numbering_definitions_part::NumberingDefinitionsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" => {
        if part.path() == "word/document.xml" {
          return Some(
                        PartRef::MainDocumentPart(
                            <crate::parts::main_document_part::MainDocumentPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.path() == "ppt/presentation.xml" {
          return Some(
                        PartRef::PresentationPart(
                            <crate::parts::presentation_part::PresentationPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.path() == "xl/workbook.xml" {
          return Some(
                        PartRef::WorkbookPart(
                            <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject" => {
        if true {
          return Some(
                        PartRef::EmbeddedObjectPart(
                            <crate::parts::embedded_object_part::EmbeddedObjectPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package" => {
        if true {
          return Some(
                        PartRef::EmbeddedPackagePart(
                            <crate::parts::embedded_package_part::EmbeddedPackagePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"
        {
          return Some(
                        PartRef::PivotTableCacheDefinitionPart(
                            <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"
        {
          return Some(
                        PartRef::PivotTableCacheRecordsPart(
                            <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"
        {
          return Some(
                        PartRef::PivotTablePart(
                            <crate::parts::pivot_table_part::PivotTablePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"
        {
          return Some(
                        PartRef::PresentationPropertiesPart(
                            <crate::parts::presentation_properties_part::PresentationPropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings"
        {
          return Some(
                        PartRef::SpreadsheetPrinterSettingsPart(
                            <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.printerSettings"
        {
          return Some(
                        PartRef::WordprocessingPrinterSettingsPart(
                            <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"
        {
          return Some(
                        PartRef::QueryTablePart(
                            <crate::parts::query_table_part::QueryTablePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData" => {
        if true {
          return Some(
                        PartRef::MailMergeRecipientDataPart(
                            <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"
        {
          return Some(
                        PartRef::WorkbookRevisionHeaderPart(
                            <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"
        {
          return Some(
                        PartRef::WorkbookRevisionLogPart(
                            <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"
        {
          return Some(
                        PartRef::DocumentSettingsPart(
                            <crate::parts::document_settings_part::DocumentSettingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"
        {
          return Some(
                        PartRef::SharedStringTablePart(
                            <crate::parts::shared_string_table_part::SharedStringTablePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/sheetMetadata" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"
        {
          return Some(
                        PartRef::CellMetadataPart(
                            <crate::parts::cell_metadata_part::CellMetadataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
        {
          return Some(
                        PartRef::SlidePart(
                            <crate::parts::slide_part::SlidePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"
        {
          return Some(
                        PartRef::SlideLayoutPart(
                            <crate::parts::slide_layout_part::SlideLayoutPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"
        {
          return Some(
                        PartRef::SlideMasterPart(
                            <crate::parts::slide_master_part::SlideMasterPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"
        {
          return Some(
                        PartRef::SlideSyncDataPart(
                            <crate::parts::slide_sync_data_part::SlideSyncDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"
        {
          return Some(
                        PartRef::StyleDefinitionsPart(
                            <crate::parts::style_definitions_part::StyleDefinitionsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"
        {
          return Some(
                        PartRef::WorkbookStylesPart(
                            <crate::parts::workbook_styles_part::WorkbookStylesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"
        {
          return Some(
                        PartRef::TableDefinitionPart(
                            <crate::parts::table_definition_part::TableDefinitionPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"
        {
          return Some(
                        PartRef::SingleCellTablePart(
                            <crate::parts::single_cell_table_part::SingleCellTablePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"
        {
          return Some(
                        PartRef::TableStylesPart(
                            <crate::parts::table_styles_part::TableStylesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.tags+xml"
        {
          return Some(
                        PartRef::UserDefinedTagsPart(
                            <crate::parts::user_defined_tags_part::UserDefinedTagsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.theme+xml" {
          return Some(
                        PartRef::ThemePart(
                            <crate::parts::theme_part::ThemePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.themeOverride+xml"
        {
          return Some(
                        PartRef::ThemeOverridePart(
                            <crate::parts::theme_override_part::ThemeOverridePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"
        {
          return Some(
                        PartRef::WorkbookUserDataPart(
                            <crate::parts::workbook_user_data_part::WorkbookUserDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"
        {
          return Some(
                        PartRef::ViewPropertiesPart(
                            <crate::parts::view_properties_part::ViewPropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing" => {
        if part.content_type() == "application/vnd.openxmlformats-officedocument.vmlDrawing" {
          return Some(
                        PartRef::VmlDrawingPart(
                            <crate::parts::vml_drawing_part::VmlDrawingPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/volatileDependencies" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"
        {
          return Some(
                        PartRef::VolatileDependenciesPart(
                            <crate::parts::volatile_dependencies_part::VolatileDependenciesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"
        {
          return Some(
                        PartRef::WebSettingsPart(
                            <crate::parts::web_settings_part::WebSettingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" => {
        if part.content_type()
          == "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"
        {
          return Some(
                        PartRef::WorksheetPart(
                            <crate::parts::worksheet_part::WorksheetPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/xmlMaps" => {
        if part.content_type() == "application/xml" {
          return Some(
                        PartRef::CustomXmlMappingsPart(
                            <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin" => {
        if part.content_type() == "application/vnd.openxmlformats-package.digital-signature-origin"
        {
          return Some(
                        PartRef::DigitalSignatureOriginPart(
                            <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature" => {
        if part.content_type()
          == "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml"
        {
          return Some(
                        PartRef::XmlSignaturePart(
                            <crate::parts::xml_signature_part::XmlSignaturePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" => {
        if part.content_type() == "application/vnd.openxmlformats-package.core-properties+xml" {
          return Some(
                        PartRef::CoreFilePropertiesPart(
                            <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail" => {
        if true {
          return Some(
                        PartRef::ThumbnailPart(
                            <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::AlternativeFormatImportPart(
                            <crate::parts::alternative_format_import_part::AlternativeFormatImportPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::CalculationChainPart(
                            <crate::parts::calculation_chain_part::CalculationChainPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::CellMetadataPart(
                            <crate::parts::cell_metadata_part::CellMetadataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::ChartColorStylePart(
                            <crate::parts::chart_color_style_part::ChartColorStylePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::ChartDrawingPart(
                            <crate::parts::chart_drawing_part::ChartDrawingPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
          return Some(
                        PartRef::ChartPart(
                            <crate::parts::chart_part::ChartPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::ChartStylePart(
                            <crate::parts::chart_style_part::ChartStylePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::ChartsheetPart(
                            <crate::parts::chartsheet_part::ChartsheetPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::CommentAuthorsPart(
                            <crate::parts::comment_authors_part::CommentAuthorsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::ConnectionsPart(
                            <crate::parts::connections_part::ConnectionsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::ControlPropertiesPart(
                            <crate::parts::control_properties_part::ControlPropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::CoreFilePropertiesPart(
                            <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::CustomDataPart(
                            <crate::parts::custom_data_part::CustomDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::CustomDataPropertiesPart(
                            <crate::parts::custom_data_properties_part::CustomDataPropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::CustomFilePropertiesPart(
                            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::CustomPropertyPart(
                            <crate::parts::custom_property_part::CustomPropertyPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::CustomXmlMappingsPart(
                            <crate::parts::custom_xml_mappings_part::CustomXmlMappingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::CustomXmlPart(
                            <crate::parts::custom_xml_part::CustomXmlPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::CustomXmlPropertiesPart(
                            <crate::parts::custom_xml_properties_part::CustomXmlPropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::CustomizationPart(
                            <crate::parts::customization_part::CustomizationPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::DiagramColorsPart(
                            <crate::parts::diagram_colors_part::DiagramColorsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::DiagramDataPart(
                            <crate::parts::diagram_data_part::DiagramDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::DiagramLayoutDefinitionPart(
                            <crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::DiagramPersistLayoutPart(
                            <crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::DiagramStylePart(
                            <crate::parts::diagram_style_part::DiagramStylePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::DialogsheetPart(
                            <crate::parts::dialogsheet_part::DialogsheetPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::DigitalSignatureOriginPart(
                            <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::DocumentSettingsPart(
                            <crate::parts::document_settings_part::DocumentSettingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::DocumentTasksPart(
                            <crate::parts::document_tasks_part::DocumentTasksPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
          return Some(
                        PartRef::DrawingsPart(
                            <crate::parts::drawings_part::DrawingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::EmbeddedControlPersistenceBinaryDataPart(
                            <crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::EmbeddedControlPersistencePart(
                            <crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::EmbeddedObjectPart(
                            <crate::parts::embedded_object_part::EmbeddedObjectPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::EmbeddedPackagePart(
                            <crate::parts::embedded_package_part::EmbeddedPackagePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
          return Some(
                        PartRef::EndnotesPart(
                            <crate::parts::endnotes_part::EndnotesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::ExcelAttachedToolbarsPart(
                            <crate::parts::excel_attached_toolbars_part::ExcelAttachedToolbarsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::ExtendedChartPart(
                            <crate::parts::extended_chart_part::ExtendedChartPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::ExtendedFilePropertiesPart(
                            <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::ExternalWorkbookPart(
                            <crate::parts::external_workbook_part::ExternalWorkbookPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::FeaturePropertyBagsPart(
                            <crate::parts::feature_property_bags_part::FeaturePropertyBagsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
          return Some(
                        PartRef::FontPart(
                            <crate::parts::font_part::FontPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::FontTablePart(
                            <crate::parts::font_table_part::FontTablePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
          return Some(
                        PartRef::FooterPart(
                            <crate::parts::footer_part::FooterPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::FootnotesPart(
                            <crate::parts::footnotes_part::FootnotesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::GlossaryDocumentPart(
                            <crate::parts::glossary_document_part::GlossaryDocumentPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::HandoutMasterPart(
                            <crate::parts::handout_master_part::HandoutMasterPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
          return Some(
                        PartRef::HeaderPart(
                            <crate::parts::header_part::HeaderPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::ImagePart(
                            <crate::parts::image_part::ImagePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::InternationalMacroSheetPart(
                            <crate::parts::international_macro_sheet_part::InternationalMacroSheetPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::LabelInfoPart(
                            <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::LegacyDiagramTextInfoPart(
                            <crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::LegacyDiagramTextPart(
                            <crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::MacroSheetPart(
                            <crate::parts::macro_sheet_part::MacroSheetPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::MailMergeRecipientDataPart(
                            <crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::MainDocumentPart(
                            <crate::parts::main_document_part::MainDocumentPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::Model3DReferenceRelationshipPart(
                            <crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::NamedSheetViewsPart(
                            <crate::parts::named_sheet_views_part::NamedSheetViewsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::NotesMasterPart(
                            <crate::parts::notes_master_part::NotesMasterPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::NotesSlidePart(
                            <crate::parts::notes_slide_part::NotesSlidePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::NumberingDefinitionsPart(
                            <crate::parts::numbering_definitions_part::NumberingDefinitionsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::PivotTableCacheDefinitionPart(
                            <crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::PivotTableCacheRecordsPart(
                            <crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::PivotTablePart(
                            <crate::parts::pivot_table_part::PivotTablePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::PowerPointAuthorsPart(
                            <crate::parts::power_point_authors_part::PowerPointAuthorsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::PowerPointCommentPart(
                            <crate::parts::power_point_comment_part::PowerPointCommentPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::PresentationPart(
                            <crate::parts::presentation_part::PresentationPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::PresentationPropertiesPart(
                            <crate::parts::presentation_properties_part::PresentationPropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::QueryTablePart(
                            <crate::parts::query_table_part::QueryTablePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::QuickAccessToolbarCustomizationsPart(
                            <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdArrayPart(
                            <crate::parts::rd_array_part::RdArrayPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdRichValuePart(
                            <crate::parts::rd_rich_value_part::RdRichValuePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdRichValueStructurePart(
                            <crate::parts::rd_rich_value_structure_part::RdRichValueStructurePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdRichValueTypesPart(
                            <crate::parts::rd_rich_value_types_part::RdRichValueTypesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdRichValueWebImagePart(
                            <crate::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdSupportingPropertyBagPart(
                            <crate::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RdSupportingPropertyBagStructurePart(
                            <crate::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RibbonAndBackstageCustomizationsPart(
                            <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::RibbonExtensibilityPart(
                            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::RichStylesPart(
                            <crate::parts::rich_styles_part::RichStylesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::SharedStringTablePart(
                            <crate::parts::shared_string_table_part::SharedStringTablePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::SingleCellTablePart(
                            <crate::parts::single_cell_table_part::SingleCellTablePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::SlicerCachePart(
                            <crate::parts::slicer_cache_part::SlicerCachePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::SlicersPart(
                            <crate::parts::slicers_part::SlicersPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::SlideCommentsPart(
                            <crate::parts::slide_comments_part::SlideCommentsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::SlideLayoutPart(
                            <crate::parts::slide_layout_part::SlideLayoutPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::SlideMasterPart(
                            <crate::parts::slide_master_part::SlideMasterPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
          return Some(
                        PartRef::SlidePart(
                            <crate::parts::slide_part::SlidePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::SlideSyncDataPart(
                            <crate::parts::slide_sync_data_part::SlideSyncDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::SpreadsheetPrinterSettingsPart(
                            <crate::parts::spreadsheet_printer_settings_part::SpreadsheetPrinterSettingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::StyleDefinitionsPart(
                            <crate::parts::style_definitions_part::StyleDefinitionsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::StylesWithEffectsPart(
                            <crate::parts::styles_with_effects_part::StylesWithEffectsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::TableDefinitionPart(
                            <crate::parts::table_definition_part::TableDefinitionPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::TableStylesPart(
                            <crate::parts::table_styles_part::TableStylesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::ThemeOverridePart(
                            <crate::parts::theme_override_part::ThemeOverridePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
          return Some(
                        PartRef::ThemePart(
                            <crate::parts::theme_part::ThemePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::ThumbnailPart(
                            <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::TimeLineCachePart(
                            <crate::parts::time_line_cache_part::TimeLineCachePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::TimeLinePart(
                            <crate::parts::time_line_part::TimeLinePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::UserDefinedTagsPart(
                            <crate::parts::user_defined_tags_part::UserDefinedTagsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
          return Some(
                        PartRef::VbaDataPart(
                            <crate::parts::vba_data_part::VbaDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
                        PartRef::VbaProjectPart(
                            <crate::parts::vba_project_part::VbaProjectPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::ViewPropertiesPart(
                            <crate::parts::view_properties_part::ViewPropertiesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::VmlDrawingPart(
                            <crate::parts::vml_drawing_part::VmlDrawingPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::VolatileDependenciesPart(
                            <crate::parts::volatile_dependencies_part::VolatileDependenciesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WebExTaskpanesPart(
                            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WebExtensionPart(
                            <crate::parts::web_extension_part::WebExtensionPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WebSettingsPart(
                            <crate::parts::web_settings_part::WebSettingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WordAttachedToolbarsPart(
                            <crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WordCommentsExtensiblePart(
                            <crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WordprocessingCommentsExPart(
                            <crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WordprocessingCommentsIdsPart(
                            <crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WordprocessingCommentsPart(
                            <crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WordprocessingPeoplePart(
                            <crate::parts::wordprocessing_people_part::WordprocessingPeoplePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WordprocessingPrinterSettingsPart(
                            <crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
          return Some(
                        PartRef::WorkbookPart(
                            <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WorkbookPersonPart(
                            <crate::parts::workbook_person_part::WorkbookPersonPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WorkbookRevisionHeaderPart(
                            <crate::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WorkbookRevisionLogPart(
                            <crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WorkbookStylesPart(
                            <crate::parts::workbook_styles_part::WorkbookStylesPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WorkbookUserDataPart(
                            <crate::parts::workbook_user_data_part::WorkbookUserDataPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WorksheetCommentsPart(
                            <crate::parts::worksheet_comments_part::WorksheetCommentsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WorksheetPart(
                            <crate::parts::worksheet_part::WorksheetPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::WorksheetSortMapPart(
                            <crate::parts::worksheet_sort_map_part::WorksheetSortMapPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
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
          return Some(
                        PartRef::WorksheetThreadedCommentsPart(
                            <crate::parts::worksheet_threaded_comments_part::WorksheetThreadedCommentsPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
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
                        PartRef::XmlSignaturePart(
                            <crate::parts::xml_signature_part::XmlSignaturePart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                                storage,
                                relationship.id(),
                                part_id,
                            ),
                        ),
                    );
        }
      }
    }
    Some(
            PartRef::ExtendedPart(
                <crate::parts::extended_part::ExtendedPart as crate::sdk::SdkPartHandle>::from_relationship_id_with_relationships(
                    storage,
                    relationship.id(),
                    part_id,
                ),
            ),
        )
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
#[derive(Clone, Debug)]
pub enum PartRootElement {
  CalculationChainPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CalculationChain,
        >,
    ),
    CellMetadataPart(
        Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Metadata>,
    ),
    #[cfg(feature = "microsoft365")]
    ChartColorStylePart(
        Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ColorStyle,
        >,
    ),
    ChartDrawingPart(
        Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes>,
    ),
    ChartPart(
        Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace>,
    ),
    #[cfg(feature = "microsoft365")]
    ChartStylePart(
        Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ChartStyle,
        >,
    ),
    ChartsheetPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Chartsheet,
        >,
    ),
    CommentAuthorsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentAuthorList,
        >,
    ),
    ConnectionsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Connections,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    ControlPropertiesPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FormControlProperties,
        >,
    ),
    CoreFilePropertiesPart(Box<crate::schemas::opc_core_properties::CoreProperties>),
    #[cfg(feature = "microsoft365")]
    CustomDataPropertiesPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem,
        >,
    ),
    CustomFilePropertiesPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_properties::Properties,
        >,
    ),
    CustomXmlMappingsPart(
        Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::MapInfo>,
    ),
    CustomXmlPropertiesPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_xml::DataStoreItem,
        >,
    ),
    CustomizationPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup,
        >,
    ),
    DiagramColorsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorsDefinition,
        >,
    ),
    DiagramDataPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot,
        >,
    ),
    DiagramLayoutDefinitionPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinition,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    DiagramPersistLayoutPart(
        Box<crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::Drawing>,
    ),
    DiagramStylePart(
        Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinition,
        >,
    ),
    DialogsheetPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet,
        >,
    ),
    DocumentSettingsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    DocumentTasksPart(
        Box<crate::schemas::schemas_microsoft_com_office_tasks_2019_documenttasks::Tasks>,
    ),
    DrawingsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
        >,
    ),
    EndnotesPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Endnotes,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    ExtendedChartPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace,
        >,
    ),
    ExtendedFilePropertiesPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties,
        >,
    ),
    ExternalWorkbookPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExternalLink,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    FeaturePropertyBagsPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag::FeaturePropertyBags,
        >,
    ),
    FontTablePart(
        Box<crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts>,
    ),
    FooterPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footer,
        >,
    ),
    FootnotesPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footnotes,
        >,
    ),
    GlossaryDocumentPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
        >,
    ),
    HandoutMasterPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::HandoutMaster,
        >,
    ),
    HeaderPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Header,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    LabelInfoPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList,
        >,
    ),
    MacroSheetPart(
        Box<crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet>,
    ),
    MainDocumentPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    NamedSheetViewsPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews::NamedSheetViews,
        >,
    ),
    NotesMasterPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesMaster,
        >,
    ),
    NotesSlidePart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesSlide,
        >,
    ),
    NumberingDefinitionsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Numbering,
        >,
    ),
    PivotTableCacheDefinitionPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
        >,
    ),
    PivotTableCacheRecordsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheRecords,
        >,
    ),
    PivotTablePart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    PowerPointAuthorsPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::AuthorList,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    PowerPointCommentPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentList,
        >,
    ),
    PresentationPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
        >,
    ),
    PresentationPropertiesPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::PresentationProperties,
        >,
    ),
    QueryTablePart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::QueryTable,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    RdArrayPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::ArrayData,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    RdRichValuePart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueData,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    RdRichValueStructurePart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueStructures,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    RdRichValueTypesPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichValueTypesInfo,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    RdRichValueWebImagePart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2020_richdatawebimage::WebImagesSupportingRichData,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    RdSupportingPropertyBagPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBags,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    RdSupportingPropertyBagStructurePart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBagStructures,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    RibbonAndBackstageCustomizationsPart(
        Box<crate::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi>,
    ),
    #[cfg(feature = "microsoft365")]
    RichStylesPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichStylesheet,
        >,
    ),
    SharedStringTablePart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SharedStringTable,
        >,
    ),
    SingleCellTablePart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SingleXmlCells,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    SlicerCachePart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheDefinition,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    SlicersPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Slicers,
        >,
    ),
    SlideCommentsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentList,
        >,
    ),
    SlideLayoutPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideLayout,
        >,
    ),
    SlideMasterPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideMaster,
        >,
    ),
    SlidePart(
        Box<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide>,
    ),
    SlideSyncDataPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideSyncProperties,
        >,
    ),
    StyleDefinitionsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    StylesWithEffectsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles,
        >,
    ),
    TableDefinitionPart(
        Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table>,
    ),
    TableStylesPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TableStyleList,
        >,
    ),
    ThemeOverridePart(
        Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ThemeOverride,
        >,
    ),
    ThemePart(
        Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Theme>,
    ),
    #[cfg(feature = "microsoft365")]
    TimeLineCachePart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    TimeLinePart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::Timelines,
        >,
    ),
    UserDefinedTagsPart(
        Box<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TagList>,
    ),
    VbaDataPart(
        Box<crate::schemas::schemas_microsoft_com_office_word_2006_wordml::VbaSuppData>,
    ),
    ViewPropertiesPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ViewProperties,
        >,
    ),
    VolatileDependenciesPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::VolatileTypes,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    WebExTaskpanesPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    WebExtensionPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension,
        >,
    ),
    WebSettingsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::WebSettings,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    WordCommentsExtensiblePart(
        Box<
            crate::schemas::schemas_microsoft_com_office_word_2018_wordml_cex::CommentsExtensible,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    WordprocessingCommentsExPart(
        Box<crate::schemas::schemas_microsoft_com_office_word_2012_wordml::CommentsEx>,
    ),
    #[cfg(feature = "microsoft365")]
    WordprocessingCommentsIdsPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_word_2016_wordml_cid::CommentsIds,
        >,
    ),
    WordprocessingCommentsPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comments,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    WordprocessingPeoplePart(
        Box<crate::schemas::schemas_microsoft_com_office_word_2012_wordml::People>,
    ),
    WorkbookPart(
        Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook>,
    ),
    #[cfg(feature = "microsoft365")]
    WorkbookPersonPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::PersonList,
        >,
    ),
    WorkbookRevisionHeaderPart(
        Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers>,
    ),
    WorkbookRevisionLogPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Revisions,
        >,
    ),
    WorkbookStylesPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Stylesheet,
        >,
    ),
    WorkbookUserDataPart(
        Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Users>,
    ),
    WorksheetCommentsPart(
        Box<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Comments>,
    ),
    WorksheetPart(
        Box<
            crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet,
        >,
    ),
    WorksheetSortMapPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_excel_2006_main::WorksheetSortMap,
        >,
    ),
    #[cfg(feature = "microsoft365")]
    WorksheetThreadedCommentsPart(
        Box<
            crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::ThreadedComments,
        >,
    ),
}
impl PartRootElement {
  pub fn part_type_name(&self) -> &'static str {
    match self {
      PartRootElement::CalculationChainPart(_) => stringify!(CalculationChainPart),
      PartRootElement::CellMetadataPart(_) => stringify!(CellMetadataPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::ChartColorStylePart(_) => stringify!(ChartColorStylePart),
      PartRootElement::ChartDrawingPart(_) => stringify!(ChartDrawingPart),
      PartRootElement::ChartPart(_) => stringify!(ChartPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::ChartStylePart(_) => stringify!(ChartStylePart),
      PartRootElement::ChartsheetPart(_) => stringify!(ChartsheetPart),
      PartRootElement::CommentAuthorsPart(_) => stringify!(CommentAuthorsPart),
      PartRootElement::ConnectionsPart(_) => stringify!(ConnectionsPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::ControlPropertiesPart(_) => {
        stringify!(ControlPropertiesPart)
      }
      PartRootElement::CoreFilePropertiesPart(_) => {
        stringify!(CoreFilePropertiesPart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::CustomDataPropertiesPart(_) => {
        stringify!(CustomDataPropertiesPart)
      }
      PartRootElement::CustomFilePropertiesPart(_) => {
        stringify!(CustomFilePropertiesPart)
      }
      PartRootElement::CustomXmlMappingsPart(_) => {
        stringify!(CustomXmlMappingsPart)
      }
      PartRootElement::CustomXmlPropertiesPart(_) => {
        stringify!(CustomXmlPropertiesPart)
      }
      PartRootElement::CustomizationPart(_) => stringify!(CustomizationPart),
      PartRootElement::DiagramColorsPart(_) => stringify!(DiagramColorsPart),
      PartRootElement::DiagramDataPart(_) => stringify!(DiagramDataPart),
      PartRootElement::DiagramLayoutDefinitionPart(_) => {
        stringify!(DiagramLayoutDefinitionPart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::DiagramPersistLayoutPart(_) => {
        stringify!(DiagramPersistLayoutPart)
      }
      PartRootElement::DiagramStylePart(_) => stringify!(DiagramStylePart),
      PartRootElement::DialogsheetPart(_) => stringify!(DialogsheetPart),
      PartRootElement::DocumentSettingsPart(_) => stringify!(DocumentSettingsPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::DocumentTasksPart(_) => stringify!(DocumentTasksPart),
      PartRootElement::DrawingsPart(_) => stringify!(DrawingsPart),
      PartRootElement::EndnotesPart(_) => stringify!(EndnotesPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::ExtendedChartPart(_) => stringify!(ExtendedChartPart),
      PartRootElement::ExtendedFilePropertiesPart(_) => {
        stringify!(ExtendedFilePropertiesPart)
      }
      PartRootElement::ExternalWorkbookPart(_) => stringify!(ExternalWorkbookPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::FeaturePropertyBagsPart(_) => {
        stringify!(FeaturePropertyBagsPart)
      }
      PartRootElement::FontTablePart(_) => stringify!(FontTablePart),
      PartRootElement::FooterPart(_) => stringify!(FooterPart),
      PartRootElement::FootnotesPart(_) => stringify!(FootnotesPart),
      PartRootElement::GlossaryDocumentPart(_) => stringify!(GlossaryDocumentPart),
      PartRootElement::HandoutMasterPart(_) => stringify!(HandoutMasterPart),
      PartRootElement::HeaderPart(_) => stringify!(HeaderPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::LabelInfoPart(_) => stringify!(LabelInfoPart),
      PartRootElement::MacroSheetPart(_) => stringify!(MacroSheetPart),
      PartRootElement::MainDocumentPart(_) => stringify!(MainDocumentPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::NamedSheetViewsPart(_) => stringify!(NamedSheetViewsPart),
      PartRootElement::NotesMasterPart(_) => stringify!(NotesMasterPart),
      PartRootElement::NotesSlidePart(_) => stringify!(NotesSlidePart),
      PartRootElement::NumberingDefinitionsPart(_) => {
        stringify!(NumberingDefinitionsPart)
      }
      PartRootElement::PivotTableCacheDefinitionPart(_) => {
        stringify!(PivotTableCacheDefinitionPart)
      }
      PartRootElement::PivotTableCacheRecordsPart(_) => {
        stringify!(PivotTableCacheRecordsPart)
      }
      PartRootElement::PivotTablePart(_) => stringify!(PivotTablePart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::PowerPointAuthorsPart(_) => {
        stringify!(PowerPointAuthorsPart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::PowerPointCommentPart(_) => {
        stringify!(PowerPointCommentPart)
      }
      PartRootElement::PresentationPart(_) => stringify!(PresentationPart),
      PartRootElement::PresentationPropertiesPart(_) => {
        stringify!(PresentationPropertiesPart)
      }
      PartRootElement::QueryTablePart(_) => stringify!(QueryTablePart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdArrayPart(_) => stringify!(RdArrayPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdRichValuePart(_) => stringify!(RdRichValuePart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdRichValueStructurePart(_) => {
        stringify!(RdRichValueStructurePart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdRichValueTypesPart(_) => stringify!(RdRichValueTypesPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdRichValueWebImagePart(_) => {
        stringify!(RdRichValueWebImagePart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdSupportingPropertyBagPart(_) => {
        stringify!(RdSupportingPropertyBagPart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdSupportingPropertyBagStructurePart(_) => {
        stringify!(RdSupportingPropertyBagStructurePart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::RibbonAndBackstageCustomizationsPart(_) => {
        stringify!(RibbonAndBackstageCustomizationsPart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::RichStylesPart(_) => stringify!(RichStylesPart),
      PartRootElement::SharedStringTablePart(_) => {
        stringify!(SharedStringTablePart)
      }
      PartRootElement::SingleCellTablePart(_) => stringify!(SingleCellTablePart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::SlicerCachePart(_) => stringify!(SlicerCachePart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::SlicersPart(_) => stringify!(SlicersPart),
      PartRootElement::SlideCommentsPart(_) => stringify!(SlideCommentsPart),
      PartRootElement::SlideLayoutPart(_) => stringify!(SlideLayoutPart),
      PartRootElement::SlideMasterPart(_) => stringify!(SlideMasterPart),
      PartRootElement::SlidePart(_) => stringify!(SlidePart),
      PartRootElement::SlideSyncDataPart(_) => stringify!(SlideSyncDataPart),
      PartRootElement::StyleDefinitionsPart(_) => stringify!(StyleDefinitionsPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::StylesWithEffectsPart(_) => {
        stringify!(StylesWithEffectsPart)
      }
      PartRootElement::TableDefinitionPart(_) => stringify!(TableDefinitionPart),
      PartRootElement::TableStylesPart(_) => stringify!(TableStylesPart),
      PartRootElement::ThemeOverridePart(_) => stringify!(ThemeOverridePart),
      PartRootElement::ThemePart(_) => stringify!(ThemePart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::TimeLineCachePart(_) => stringify!(TimeLineCachePart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::TimeLinePart(_) => stringify!(TimeLinePart),
      PartRootElement::UserDefinedTagsPart(_) => stringify!(UserDefinedTagsPart),
      PartRootElement::VbaDataPart(_) => stringify!(VbaDataPart),
      PartRootElement::ViewPropertiesPart(_) => stringify!(ViewPropertiesPart),
      PartRootElement::VolatileDependenciesPart(_) => {
        stringify!(VolatileDependenciesPart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::WebExTaskpanesPart(_) => stringify!(WebExTaskpanesPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WebExtensionPart(_) => stringify!(WebExtensionPart),
      PartRootElement::WebSettingsPart(_) => stringify!(WebSettingsPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WordCommentsExtensiblePart(_) => {
        stringify!(WordCommentsExtensiblePart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::WordprocessingCommentsExPart(_) => {
        stringify!(WordprocessingCommentsExPart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::WordprocessingCommentsIdsPart(_) => {
        stringify!(WordprocessingCommentsIdsPart)
      }
      PartRootElement::WordprocessingCommentsPart(_) => {
        stringify!(WordprocessingCommentsPart)
      }
      #[cfg(feature = "microsoft365")]
      PartRootElement::WordprocessingPeoplePart(_) => {
        stringify!(WordprocessingPeoplePart)
      }
      PartRootElement::WorkbookPart(_) => stringify!(WorkbookPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WorkbookPersonPart(_) => stringify!(WorkbookPersonPart),
      PartRootElement::WorkbookRevisionHeaderPart(_) => {
        stringify!(WorkbookRevisionHeaderPart)
      }
      PartRootElement::WorkbookRevisionLogPart(_) => {
        stringify!(WorkbookRevisionLogPart)
      }
      PartRootElement::WorkbookStylesPart(_) => stringify!(WorkbookStylesPart),
      PartRootElement::WorkbookUserDataPart(_) => stringify!(WorkbookUserDataPart),
      PartRootElement::WorksheetCommentsPart(_) => {
        stringify!(WorksheetCommentsPart)
      }
      PartRootElement::WorksheetPart(_) => stringify!(WorksheetPart),
      PartRootElement::WorksheetSortMapPart(_) => stringify!(WorksheetSortMapPart),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WorksheetThreadedCommentsPart(_) => {
        stringify!(WorksheetThreadedCommentsPart)
      }
    }
  }
  pub fn as_calculation_chain_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CalculationChain>
  {
    match self {
      PartRootElement::CalculationChainPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_calculation_chain_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CalculationChain,
  > {
    match self {
      PartRootElement::CalculationChainPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_cell_metadata_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Metadata> {
    match self {
      PartRootElement::CellMetadataPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_cell_metadata_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Metadata> {
    match self {
      PartRootElement::CellMetadataPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_chart_color_style_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ColorStyle>
  {
    match self {
      PartRootElement::ChartColorStylePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_chart_color_style_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ColorStyle>
  {
    match self {
      PartRootElement::ChartColorStylePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_chart_drawing_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes> {
    match self {
      PartRootElement::ChartDrawingPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_chart_drawing_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes> {
    match self {
      PartRootElement::ChartDrawingPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_chart_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace> {
    match self {
      PartRootElement::ChartPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_chart_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace> {
    match self {
      PartRootElement::ChartPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_chart_style_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ChartStyle>
  {
    match self {
      PartRootElement::ChartStylePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_chart_style_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ChartStyle>
  {
    match self {
      PartRootElement::ChartStylePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_chartsheet_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Chartsheet> {
    match self {
      PartRootElement::ChartsheetPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_chartsheet_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Chartsheet>
  {
    match self {
      PartRootElement::ChartsheetPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_comment_authors_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentAuthorList>
  {
    match self {
      PartRootElement::CommentAuthorsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_comment_authors_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentAuthorList,
  > {
    match self {
      PartRootElement::CommentAuthorsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_connections_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Connections> {
    match self {
      PartRootElement::ConnectionsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_connections_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Connections>
  {
    match self {
      PartRootElement::ConnectionsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_control_properties_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FormControlProperties,
  > {
    match self {
      PartRootElement::ControlPropertiesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_control_properties_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FormControlProperties,
  >{
    match self {
      PartRootElement::ControlPropertiesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_core_file_properties_part(
    &self,
  ) -> Option<&crate::schemas::opc_core_properties::CoreProperties> {
    match self {
      PartRootElement::CoreFilePropertiesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_core_file_properties_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::opc_core_properties::CoreProperties> {
    match self {
      PartRootElement::CoreFilePropertiesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_custom_data_properties_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem>
  {
    match self {
      PartRootElement::CustomDataPropertiesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_custom_data_properties_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem,
  > {
    match self {
      PartRootElement::CustomDataPropertiesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_custom_file_properties_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_properties::Properties,
  > {
    match self {
      PartRootElement::CustomFilePropertiesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
    pub fn as_custom_file_properties_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_properties::Properties,
  >{
    match self {
      PartRootElement::CustomFilePropertiesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_custom_xml_mappings_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::MapInfo> {
    match self {
      PartRootElement::CustomXmlMappingsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_custom_xml_mappings_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::MapInfo> {
    match self {
      PartRootElement::CustomXmlMappingsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_custom_xml_properties_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_xml::DataStoreItem,
  > {
    match self {
      PartRootElement::CustomXmlPropertiesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_custom_xml_properties_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_xml::DataStoreItem,
  > {
    match self {
      PartRootElement::CustomXmlPropertiesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_customization_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup>
  {
    match self {
      PartRootElement::CustomizationPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_customization_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup,
  > {
    match self {
      PartRootElement::CustomizationPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_diagram_colors_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorsDefinition>
  {
    match self {
      PartRootElement::DiagramColorsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_diagram_colors_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorsDefinition,
  > {
    match self {
      PartRootElement::DiagramColorsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_diagram_data_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot> {
    match self {
      PartRootElement::DiagramDataPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_diagram_data_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot>
  {
    match self {
      PartRootElement::DiagramDataPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_diagram_layout_definition_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinition>
  {
    match self {
      PartRootElement::DiagramLayoutDefinitionPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_diagram_layout_definition_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinition,
  > {
    match self {
      PartRootElement::DiagramLayoutDefinitionPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_diagram_persist_layout_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::Drawing> {
    match self {
      PartRootElement::DiagramPersistLayoutPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_diagram_persist_layout_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::Drawing> {
    match self {
      PartRootElement::DiagramPersistLayoutPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_diagram_style_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinition>
  {
    match self {
      PartRootElement::DiagramStylePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_diagram_style_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinition>
  {
    match self {
      PartRootElement::DiagramStylePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_dialogsheet_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet> {
    match self {
      PartRootElement::DialogsheetPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_dialogsheet_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet>
  {
    match self {
      PartRootElement::DialogsheetPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_document_settings_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings> {
    match self {
      PartRootElement::DocumentSettingsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_document_settings_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings>
  {
    match self {
      PartRootElement::DocumentSettingsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_document_tasks_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_tasks_2019_documenttasks::Tasks> {
    match self {
      PartRootElement::DocumentTasksPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_document_tasks_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_tasks_2019_documenttasks::Tasks> {
    match self {
      PartRootElement::DocumentTasksPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
    pub fn as_drawings_part(
        &self,
    ) -> Option<
        &crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
  >{
    match self {
      PartRootElement::DrawingsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
    pub fn as_drawings_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
  >{
    match self {
      PartRootElement::DrawingsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_endnotes_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Endnotes> {
    match self {
      PartRootElement::EndnotesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_endnotes_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Endnotes>
  {
    match self {
      PartRootElement::EndnotesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_extended_chart_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace> {
    match self {
      PartRootElement::ExtendedChartPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_extended_chart_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace>
  {
    match self {
      PartRootElement::ExtendedChartPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
    pub fn as_extended_file_properties_part(
        &self,
    ) -> Option<
        &crate::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties,
  >{
    match self {
      PartRootElement::ExtendedFilePropertiesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
    pub fn as_extended_file_properties_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties,
  >{
    match self {
      PartRootElement::ExtendedFilePropertiesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_external_workbook_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExternalLink> {
    match self {
      PartRootElement::ExternalWorkbookPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_external_workbook_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExternalLink>
  {
    match self {
      PartRootElement::ExternalWorkbookPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_feature_property_bags_part(
        &self,
    ) -> Option<
        &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag::FeaturePropertyBags,
  >{
    match self {
      PartRootElement::FeaturePropertyBagsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_feature_property_bags_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag::FeaturePropertyBags,
  >{
    match self {
      PartRootElement::FeaturePropertyBagsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_font_table_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts> {
    match self {
      PartRootElement::FontTablePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_font_table_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts> {
    match self {
      PartRootElement::FontTablePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_footer_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footer> {
    match self {
      PartRootElement::FooterPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_footer_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footer>
  {
    match self {
      PartRootElement::FooterPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_footnotes_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footnotes> {
    match self {
      PartRootElement::FootnotesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_footnotes_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footnotes>
  {
    match self {
      PartRootElement::FootnotesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_glossary_document_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
  > {
    match self {
      PartRootElement::GlossaryDocumentPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_glossary_document_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
  > {
    match self {
      PartRootElement::GlossaryDocumentPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_handout_master_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::HandoutMaster>
  {
    match self {
      PartRootElement::HandoutMasterPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_handout_master_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::HandoutMaster>
  {
    match self {
      PartRootElement::HandoutMasterPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_header_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Header> {
    match self {
      PartRootElement::HeaderPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_header_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Header>
  {
    match self {
      PartRootElement::HeaderPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_label_info_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList,
  > {
    match self {
      PartRootElement::LabelInfoPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_label_info_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList,
  >{
    match self {
      PartRootElement::LabelInfoPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_macro_sheet_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet> {
    match self {
      PartRootElement::MacroSheetPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_macro_sheet_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet> {
    match self {
      PartRootElement::MacroSheetPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_main_document_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document> {
    match self {
      PartRootElement::MainDocumentPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_main_document_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document>
  {
    match self {
      PartRootElement::MainDocumentPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_named_sheet_views_part(
        &self,
    ) -> Option<
        &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews::NamedSheetViews,
  >{
    match self {
      PartRootElement::NamedSheetViewsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_named_sheet_views_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews::NamedSheetViews,
  >{
    match self {
      PartRootElement::NamedSheetViewsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_notes_master_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesMaster> {
    match self {
      PartRootElement::NotesMasterPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_notes_master_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesMaster>
  {
    match self {
      PartRootElement::NotesMasterPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_notes_slide_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesSlide> {
    match self {
      PartRootElement::NotesSlidePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_notes_slide_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesSlide>
  {
    match self {
      PartRootElement::NotesSlidePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_numbering_definitions_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Numbering> {
    match self {
      PartRootElement::NumberingDefinitionsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_numbering_definitions_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Numbering>
  {
    match self {
      PartRootElement::NumberingDefinitionsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_pivot_table_cache_definition_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
  > {
    match self {
      PartRootElement::PivotTableCacheDefinitionPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_pivot_table_cache_definition_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
  > {
    match self {
      PartRootElement::PivotTableCacheDefinitionPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_pivot_table_cache_records_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheRecords>
  {
    match self {
      PartRootElement::PivotTableCacheRecordsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_pivot_table_cache_records_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheRecords,
  > {
    match self {
      PartRootElement::PivotTableCacheRecordsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_pivot_table_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition,
  > {
    match self {
      PartRootElement::PivotTablePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_pivot_table_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition,
  > {
    match self {
      PartRootElement::PivotTablePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_power_point_authors_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::AuthorList> {
    match self {
      PartRootElement::PowerPointAuthorsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_power_point_authors_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::AuthorList>
  {
    match self {
      PartRootElement::PowerPointAuthorsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_power_point_comment_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentList> {
    match self {
      PartRootElement::PowerPointCommentPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_power_point_comment_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentList>
  {
    match self {
      PartRootElement::PowerPointCommentPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_presentation_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation>
  {
    match self {
      PartRootElement::PresentationPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_presentation_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation>
  {
    match self {
      PartRootElement::PresentationPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_presentation_properties_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::PresentationProperties,
  > {
    match self {
      PartRootElement::PresentationPropertiesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
    pub fn as_presentation_properties_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::PresentationProperties,
  >{
    match self {
      PartRootElement::PresentationPropertiesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_query_table_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::QueryTable> {
    match self {
      PartRootElement::QueryTablePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_query_table_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::QueryTable>
  {
    match self {
      PartRootElement::QueryTablePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_rd_array_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::ArrayData>
  {
    match self {
      PartRootElement::RdArrayPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_rd_array_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::ArrayData,
  > {
    match self {
      PartRootElement::RdArrayPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_rd_rich_value_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueData,
  > {
    match self {
      PartRootElement::RdRichValuePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_rd_rich_value_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueData,
  > {
    match self {
      PartRootElement::RdRichValuePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_rd_rich_value_structure_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueStructures,
  > {
    match self {
      PartRootElement::RdRichValueStructurePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_rd_rich_value_structure_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueStructures,
  >{
    match self {
      PartRootElement::RdRichValueStructurePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_rd_rich_value_types_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichValueTypesInfo,
  > {
    match self {
      PartRootElement::RdRichValueTypesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_rd_rich_value_types_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichValueTypesInfo,
  >{
    match self {
      PartRootElement::RdRichValueTypesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_rd_rich_value_web_image_part(
        &self,
    ) -> Option<
        &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2020_richdatawebimage::WebImagesSupportingRichData,
  >{
    match self {
      PartRootElement::RdRichValueWebImagePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_rd_rich_value_web_image_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2020_richdatawebimage::WebImagesSupportingRichData,
  >{
    match self {
      PartRootElement::RdRichValueWebImagePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_rd_supporting_property_bag_part(
        &self,
    ) -> Option<
        &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBags,
  >{
    match self {
      PartRootElement::RdSupportingPropertyBagPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_rd_supporting_property_bag_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBags,
  >{
    match self {
      PartRootElement::RdSupportingPropertyBagPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_rd_supporting_property_bag_structure_part(
        &self,
    ) -> Option<
        &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBagStructures,
  >{
    match self {
      PartRootElement::RdSupportingPropertyBagStructurePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_rd_supporting_property_bag_structure_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBagStructures,
  >{
    match self {
      PartRootElement::RdSupportingPropertyBagStructurePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_ribbon_and_backstage_customizations_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi> {
    match self {
      PartRootElement::RibbonAndBackstageCustomizationsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_ribbon_and_backstage_customizations_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi> {
    match self {
      PartRootElement::RibbonAndBackstageCustomizationsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_rich_styles_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichStylesheet,
  > {
    match self {
      PartRootElement::RichStylesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_rich_styles_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichStylesheet,
  > {
    match self {
      PartRootElement::RichStylesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_shared_string_table_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SharedStringTable>
  {
    match self {
      PartRootElement::SharedStringTablePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_shared_string_table_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SharedStringTable,
  > {
    match self {
      PartRootElement::SharedStringTablePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_single_cell_table_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SingleXmlCells>
  {
    match self {
      PartRootElement::SingleCellTablePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_single_cell_table_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SingleXmlCells>
  {
    match self {
      PartRootElement::SingleCellTablePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_slicer_cache_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheDefinition,
  > {
    match self {
      PartRootElement::SlicerCachePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_slicer_cache_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheDefinition,
  >{
    match self {
      PartRootElement::SlicerCachePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_slicers_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Slicers> {
    match self {
      PartRootElement::SlicersPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_slicers_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Slicers>
  {
    match self {
      PartRootElement::SlicersPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_slide_comments_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentList> {
    match self {
      PartRootElement::SlideCommentsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_slide_comments_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentList>
  {
    match self {
      PartRootElement::SlideCommentsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_slide_layout_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideLayout> {
    match self {
      PartRootElement::SlideLayoutPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_slide_layout_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideLayout>
  {
    match self {
      PartRootElement::SlideLayoutPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_slide_master_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideMaster> {
    match self {
      PartRootElement::SlideMasterPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_slide_master_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideMaster>
  {
    match self {
      PartRootElement::SlideMasterPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_slide_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide> {
    match self {
      PartRootElement::SlidePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_slide_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide> {
    match self {
      PartRootElement::SlidePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_slide_sync_data_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideSyncProperties,
  > {
    match self {
      PartRootElement::SlideSyncDataPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_slide_sync_data_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideSyncProperties,
  > {
    match self {
      PartRootElement::SlideSyncDataPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_style_definitions_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles> {
    match self {
      PartRootElement::StyleDefinitionsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_style_definitions_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles>
  {
    match self {
      PartRootElement::StyleDefinitionsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_styles_with_effects_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles> {
    match self {
      PartRootElement::StylesWithEffectsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_styles_with_effects_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles>
  {
    match self {
      PartRootElement::StylesWithEffectsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_table_definition_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table> {
    match self {
      PartRootElement::TableDefinitionPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_table_definition_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table> {
    match self {
      PartRootElement::TableDefinitionPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_table_styles_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TableStyleList> {
    match self {
      PartRootElement::TableStylesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_table_styles_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TableStyleList>
  {
    match self {
      PartRootElement::TableStylesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_theme_override_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ThemeOverride> {
    match self {
      PartRootElement::ThemeOverridePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_theme_override_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ThemeOverride>
  {
    match self {
      PartRootElement::ThemeOverridePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_theme_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Theme> {
    match self {
      PartRootElement::ThemePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_theme_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Theme> {
    match self {
      PartRootElement::ThemePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_time_line_cache_part(
        &self,
    ) -> Option<
        &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition,
  >{
    match self {
      PartRootElement::TimeLineCachePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_time_line_cache_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition,
  >{
    match self {
      PartRootElement::TimeLineCachePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_time_line_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::Timelines>
  {
    match self {
      PartRootElement::TimeLinePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_time_line_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::Timelines>
  {
    match self {
      PartRootElement::TimeLinePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_user_defined_tags_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TagList> {
    match self {
      PartRootElement::UserDefinedTagsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_user_defined_tags_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TagList> {
    match self {
      PartRootElement::UserDefinedTagsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_vba_data_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_word_2006_wordml::VbaSuppData> {
    match self {
      PartRootElement::VbaDataPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_vba_data_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_word_2006_wordml::VbaSuppData> {
    match self {
      PartRootElement::VbaDataPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_view_properties_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ViewProperties>
  {
    match self {
      PartRootElement::ViewPropertiesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_view_properties_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ViewProperties,
  > {
    match self {
      PartRootElement::ViewPropertiesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_volatile_dependencies_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::VolatileTypes>
  {
    match self {
      PartRootElement::VolatileDependenciesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_volatile_dependencies_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::VolatileTypes>
  {
    match self {
      PartRootElement::VolatileDependenciesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_web_ex_taskpanes_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes,
  > {
    match self {
      PartRootElement::WebExTaskpanesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_web_ex_taskpanes_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes,
  > {
    match self {
      PartRootElement::WebExTaskpanesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_web_extension_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension,
  > {
    match self {
      PartRootElement::WebExtensionPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_web_extension_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension,
  >{
    match self {
      PartRootElement::WebExtensionPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_web_settings_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::WebSettings>
  {
    match self {
      PartRootElement::WebSettingsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_web_settings_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::WebSettings>
  {
    match self {
      PartRootElement::WebSettingsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_word_comments_extensible_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_word_2018_wordml_cex::CommentsExtensible>
  {
    match self {
      PartRootElement::WordCommentsExtensiblePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_word_comments_extensible_part_mut(
    &mut self,
  ) -> Option<
    &mut crate::schemas::schemas_microsoft_com_office_word_2018_wordml_cex::CommentsExtensible,
  > {
    match self {
      PartRootElement::WordCommentsExtensiblePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_wordprocessing_comments_ex_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_word_2012_wordml::CommentsEx> {
    match self {
      PartRootElement::WordprocessingCommentsExPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_wordprocessing_comments_ex_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_word_2012_wordml::CommentsEx> {
    match self {
      PartRootElement::WordprocessingCommentsExPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_wordprocessing_comments_ids_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_word_2016_wordml_cid::CommentsIds> {
    match self {
      PartRootElement::WordprocessingCommentsIdsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_wordprocessing_comments_ids_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_word_2016_wordml_cid::CommentsIds>
  {
    match self {
      PartRootElement::WordprocessingCommentsIdsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_wordprocessing_comments_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comments> {
    match self {
      PartRootElement::WordprocessingCommentsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_wordprocessing_comments_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comments>
  {
    match self {
      PartRootElement::WordprocessingCommentsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_wordprocessing_people_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_word_2012_wordml::People> {
    match self {
      PartRootElement::WordprocessingPeoplePart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_wordprocessing_people_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_word_2012_wordml::People> {
    match self {
      PartRootElement::WordprocessingPeoplePart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_workbook_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook> {
    match self {
      PartRootElement::WorkbookPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_workbook_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook> {
    match self {
      PartRootElement::WorkbookPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
  pub fn as_workbook_person_part(
    &self,
  ) -> Option<
    &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::PersonList,
  > {
    match self {
      PartRootElement::WorkbookPersonPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_workbook_person_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::PersonList,
  >{
    match self {
      PartRootElement::WorkbookPersonPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_workbook_revision_header_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers> {
    match self {
      PartRootElement::WorkbookRevisionHeaderPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_workbook_revision_header_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers> {
    match self {
      PartRootElement::WorkbookRevisionHeaderPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_workbook_revision_log_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Revisions> {
    match self {
      PartRootElement::WorkbookRevisionLogPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_workbook_revision_log_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Revisions>
  {
    match self {
      PartRootElement::WorkbookRevisionLogPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_workbook_styles_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Stylesheet> {
    match self {
      PartRootElement::WorkbookStylesPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_workbook_styles_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Stylesheet>
  {
    match self {
      PartRootElement::WorkbookStylesPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_workbook_user_data_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Users> {
    match self {
      PartRootElement::WorkbookUserDataPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_workbook_user_data_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Users> {
    match self {
      PartRootElement::WorkbookUserDataPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_worksheet_comments_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Comments> {
    match self {
      PartRootElement::WorksheetCommentsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_worksheet_comments_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Comments> {
    match self {
      PartRootElement::WorksheetCommentsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_worksheet_part(
    &self,
  ) -> Option<&crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet> {
    match self {
      PartRootElement::WorksheetPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_worksheet_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet>
  {
    match self {
      PartRootElement::WorksheetPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn as_worksheet_sort_map_part(
    &self,
  ) -> Option<&crate::schemas::schemas_microsoft_com_office_excel_2006_main::WorksheetSortMap> {
    match self {
      PartRootElement::WorksheetSortMapPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  pub fn as_worksheet_sort_map_part_mut(
    &mut self,
  ) -> Option<&mut crate::schemas::schemas_microsoft_com_office_excel_2006_main::WorksheetSortMap>
  {
    match self {
      PartRootElement::WorksheetSortMapPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_worksheet_threaded_comments_part(
        &self,
    ) -> Option<
        &crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::ThreadedComments,
  >{
    match self {
      PartRootElement::WorksheetThreadedCommentsPart(root) => Some(root.as_ref()),
      _ => None,
    }
  }
  #[cfg(feature = "microsoft365")]
    pub fn as_worksheet_threaded_comments_part_mut(
        &mut self,
    ) -> Option<
        &mut crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::ThreadedComments,
  >{
    match self {
      PartRootElement::WorksheetThreadedCommentsPart(root) => Some(root.as_mut()),
      _ => None,
    }
  }
  pub fn to_xml_bytes(&self) -> Result<Vec<u8>, crate::common::SdkError> {
    match self {
      PartRootElement::CalculationChainPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::CellMetadataPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::ChartColorStylePart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::ChartDrawingPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::ChartPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::ChartStylePart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::ChartsheetPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::CommentAuthorsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::ConnectionsPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::ControlPropertiesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::CoreFilePropertiesPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::CustomDataPropertiesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::CustomFilePropertiesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::CustomXmlMappingsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::CustomXmlPropertiesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::CustomizationPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::DiagramColorsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::DiagramDataPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::DiagramLayoutDefinitionPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::DiagramPersistLayoutPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::DiagramStylePart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::DialogsheetPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::DocumentSettingsPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::DocumentTasksPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::DrawingsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::EndnotesPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::ExtendedChartPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::ExtendedFilePropertiesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::ExternalWorkbookPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::FeaturePropertyBagsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::FontTablePart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::FooterPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::FootnotesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::GlossaryDocumentPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::HandoutMasterPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::HeaderPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::LabelInfoPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::MacroSheetPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::MainDocumentPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::NamedSheetViewsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::NotesMasterPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::NotesSlidePart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::NumberingDefinitionsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::PivotTableCacheDefinitionPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::PivotTableCacheRecordsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::PivotTablePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::PowerPointAuthorsPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::PowerPointCommentPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::PresentationPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::PresentationPropertiesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::QueryTablePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdArrayPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdRichValuePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdRichValueStructurePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdRichValueTypesPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdRichValueWebImagePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdSupportingPropertyBagPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RdSupportingPropertyBagStructurePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RibbonAndBackstageCustomizationsPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::RichStylesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::SharedStringTablePart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::SingleCellTablePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::SlicerCachePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::SlicersPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::SlideCommentsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::SlideLayoutPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::SlideMasterPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::SlidePart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::SlideSyncDataPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::StyleDefinitionsPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::StylesWithEffectsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::TableDefinitionPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::TableStylesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::ThemeOverridePart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::ThemePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::TimeLineCachePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::TimeLinePart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::UserDefinedTagsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::VbaDataPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::ViewPropertiesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::VolatileDependenciesPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WebExTaskpanesPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WebExtensionPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::WebSettingsPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WordCommentsExtensiblePart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WordprocessingCommentsExPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WordprocessingCommentsIdsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::WordprocessingCommentsPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WordprocessingPeoplePart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::WorkbookPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WorkbookPersonPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::WorkbookRevisionHeaderPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::WorkbookRevisionLogPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::WorkbookStylesPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::WorkbookUserDataPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::WorksheetCommentsPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::WorksheetPart(root) => Ok(root.to_xml_bytes()?),
      PartRootElement::WorksheetSortMapPart(root) => Ok(root.to_xml_bytes()?),
      #[cfg(feature = "microsoft365")]
      PartRootElement::WorksheetThreadedCommentsPart(root) => Ok(root.to_xml_bytes()?),
    }
  }
  pub fn from_part_id(
    storage: &crate::common::SdkPackageStorage,
    part_id: crate::common::PartId,
  ) -> Result<Option<Self>, crate::common::SdkError> {
    let Some(part) = storage.part(part_id) else {
      return Ok(None);
    };
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CalculationChain::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::CalculationChainPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Metadata::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::CellMetadataPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-office.chartcolorstyle+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-office.chartcolorstyle+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ColorStyle::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ChartColorStylePart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ChartDrawingPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.drawingml.chart+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ChartPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-office.chartstyle+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-office.chartstyle+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ChartStyle::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ChartStylePart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Chartsheet::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ChartsheetPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentAuthorList::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::CommentAuthorsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Connections::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ConnectionsPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.controlproperties+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.controlproperties+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FormControlProperties::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ControlPropertiesPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-package.core-properties+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.openxmlformats-package.core-properties+xml"
    {
      return crate::schemas::opc_core_properties::CoreProperties::from_bytes(part.data().bytes())
        .map(Box::new)
        .map(PartRootElement::CoreFilePropertiesPart)
        .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.customDataProperties+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.customDataProperties+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::CustomDataPropertiesPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.custom-properties+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.custom-properties+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_properties::Properties::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::CustomFilePropertiesPart)
                .map(Some);
    }
    if !matches!("application/xml", "" | "application/xml" | "text/xml")
      && part.content_type() == "application/xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::MapInfo::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::CustomXmlMappingsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.customXmlProperties+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.customXmlProperties+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_office_document_2006_custom_xml::DataStoreItem::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::CustomXmlPropertiesPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.ms-word.keyMapCustomizations+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-word.keyMapCustomizations+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::CustomizationPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::ColorsDefinition::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::DiagramColorsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::DiagramDataPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinition::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::DiagramLayoutDefinitionPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-office.drawingml.diagramDrawing+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-office.drawingml.diagramDrawing+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::Drawing::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::DiagramPersistLayoutPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinition::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::DiagramStylePart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::DialogSheet::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::DialogsheetPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::DocumentSettingsPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-office.documenttasks+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-office.documenttasks+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_tasks_2019_documenttasks::Tasks::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::DocumentTasksPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.drawing+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.openxmlformats-officedocument.drawing+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::DrawingsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Endnotes::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::EndnotesPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-office.chartex+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-office.chartex+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ExtendedChartPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.extended-properties+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.extended-properties+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ExtendedFilePropertiesPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExternalLink::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ExternalWorkbookPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.featurepropertybag+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.featurepropertybag+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag::FeaturePropertyBags::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::FeaturePropertyBagsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::FontTablePart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footer::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::FooterPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footnotes::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::FootnotesPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::GlossaryDocumentPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::HandoutMaster::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::HandoutMasterPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Header::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::HeaderPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-office.classificationlabels+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-office.classificationlabels+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::LabelInfoPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.ms-excel.macrosheet+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.macrosheet+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_excel_2006_main::Macrosheet::from_bytes(
        part.data().bytes(),
      )
      .map(Box::new)
      .map(PartRootElement::MacroSheetPart)
      .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::MainDocumentPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.namedsheetviews+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.namedsheetviews+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews::NamedSheetViews::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::NamedSheetViewsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesMaster::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::NotesMasterPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesSlide::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::NotesSlidePart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Numbering::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::NumberingDefinitionsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::PivotTableCacheDefinitionPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheRecords::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::PivotTableCacheRecordsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::PivotTablePart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-powerpoint.authors+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-powerpoint.authors+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::AuthorList::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::PowerPointAuthorsPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-powerpoint.comments+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-powerpoint.comments+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::CommentList::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::PowerPointCommentPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::PresentationPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::PresentationProperties::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::PresentationPropertiesPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::QueryTable::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::QueryTablePart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.rdarray+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.rdarray+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::ArrayData::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::RdArrayPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.rdrichvalue+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.rdrichvalue+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueData::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::RdRichValuePart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.rdrichvaluestructure+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.rdrichvaluestructure+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueStructures::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::RdRichValueStructurePart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.rdrichvaluetypes+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.rdrichvaluetypes+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichValueTypesInfo::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::RdRichValueTypesPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.rdrichvaluewebimage+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.rdrichvaluewebimage+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2020_richdatawebimage::WebImagesSupportingRichData::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::RdRichValueWebImagePart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.rdsupportingpropertybag+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.rdsupportingpropertybag+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBags::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::RdSupportingPropertyBagPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBagStructures::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::RdSupportingPropertyBagStructurePart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!("application/xml", "" | "application/xml" | "text/xml")
      && part.content_type() == "application/xml"
    {
      return crate::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi::from_bytes(
        part.data().bytes(),
      )
      .map(Box::new)
      .map(PartRootElement::RibbonAndBackstageCustomizationsPart)
      .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.richstyles+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.richstyles+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichStylesheet::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::RichStylesPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SharedStringTable::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::SharedStringTablePart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SingleXmlCells::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::SingleCellTablePart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.slicerCache+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.slicerCache+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::SlicerCacheDefinition::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::SlicerCachePart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.slicer+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.slicer+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Slicers::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::SlicersPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.comments+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.comments+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CommentList::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::SlideCommentsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideLayout::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::SlideLayoutPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideMaster::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::SlideMasterPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.slide+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::SlidePart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideSyncProperties::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::SlideSyncDataPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::StyleDefinitionsPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-word.stylesWithEffects+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-word.stylesWithEffects+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Styles::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::StylesWithEffectsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::TableDefinitionPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TableStyleList::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::TableStylesPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.themeOverride+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.openxmlformats-officedocument.themeOverride+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ThemeOverride::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ThemeOverridePart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.theme+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.openxmlformats-officedocument.theme+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Theme::from_bytes(
        part.data().bytes(),
      )
      .map(Box::new)
      .map(PartRootElement::ThemePart)
      .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.timelineCache+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.timelineCache+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::TimelineCacheDefinition::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::TimeLineCachePart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.timeline+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.timeline+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main::Timelines::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::TimeLinePart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.tags+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.tags+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TagList::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::UserDefinedTagsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.ms-word.vbaData+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-word.vbaData+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_word_2006_wordml::VbaSuppData::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::VbaDataPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ViewProperties::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::ViewPropertiesPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::VolatileTypes::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::VolatileDependenciesPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-office.webextensiontaskpanes+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-office.webextensiontaskpanes+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WebExTaskpanesPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-office.webextension+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-office.webextension+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WebExtensionPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::WebSettings::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WebSettingsPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtensible+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_word_2018_wordml_cex::CommentsExtensible::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WordCommentsExtensiblePart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_word_2012_wordml::CommentsEx::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WordprocessingCommentsExPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsIds+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_word_2016_wordml_cid::CommentsIds::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WordprocessingCommentsIdsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comments::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WordprocessingCommentsPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_word_2012_wordml::People::from_bytes(
        part.data().bytes(),
      )
      .map(Box::new)
      .map(PartRootElement::WordprocessingPeoplePart)
      .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WorkbookPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.person+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.person+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::PersonList::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WorkbookPersonPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WorkbookRevisionHeaderPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Revisions::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WorkbookRevisionLogPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Stylesheet::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WorkbookStylesPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Users::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WorkbookUserDataPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Comments::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WorksheetCommentsPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type()
      == "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"
    {
      return crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Worksheet::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WorksheetPart)
                .map(Some);
    }
    if !matches!(
      "application/vnd.ms-excel.wsSortMap+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.wsSortMap+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_excel_2006_main::WorksheetSortMap::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WorksheetSortMapPart)
                .map(Some);
    }
    #[cfg(feature = "microsoft365")]
    if !matches!(
      "application/vnd.ms-excel.threadedcomments+xml",
      "" | "application/xml" | "text/xml"
    ) && part.content_type() == "application/vnd.ms-excel.threadedcomments+xml"
    {
      return crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::ThreadedComments::from_bytes(
                    part.data().bytes(),
                )
                .map(Box::new)
                .map(PartRootElement::WorksheetThreadedCommentsPart)
                .map(Some);
    }
    Ok(None)
  }
}
pub fn initialize_root_elements(
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
pub trait PartRootCache: crate::sdk::SdkPackage {
  fn root_element(&self, part_id: crate::common::PartId) -> Option<&crate::parts::PartRootElement>;
  fn root_element_slot_mut(
    &mut self,
    part_id: crate::common::PartId,
  ) -> Option<&mut Option<crate::parts::PartRootElement>>;
  fn push_root_element_slot(&mut self);
  #[inline]
  fn is_root_element_loaded(&self, part_id: crate::common::PartId) -> bool {
    self.root_element(part_id).is_some()
  }
  #[inline]
  fn unload_root_element(
    &mut self,
    part_id: crate::common::PartId,
  ) -> Option<crate::parts::PartRootElement> {
    self.root_element_slot_mut(part_id)?.take()
  }
  #[inline]
  fn part_bytes_for_copy(
    &self,
    part_id: crate::common::PartId,
  ) -> Result<Vec<u8>, crate::common::SdkError> {
    if let Some(root_element) = self.root_element(part_id) {
      root_element.to_xml_bytes()
    } else {
      let part = crate::sdk::SdkPackageInternal::storage(self)
        .part(part_id)
        .ok_or_else(|| {
          crate::common::SdkError::CommonError(format!(
            "part id {part_id:?} is not present in package storage"
          ))
        })?;
      Ok(part.data().bytes().to_vec())
    }
  }
}
pub fn save_package<P, W>(package: &P, writer: W) -> Result<(), crate::common::SdkError>
where
  P: crate::parts::PartRootCache,
  W: std::io::Write + std::io::Seek,
{
  use std::io::Write as _;
  let mut zip = zip::ZipWriter::new(writer);
  let options = zip::write::SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Deflated)
    .unix_permissions(0o755);
  let mut entry_set = std::collections::HashSet::<String>::new();
  let storage = crate::sdk::SdkPackageInternal::storage(package);
  let mut modeled_part_relationships = std::collections::HashMap::new();
  crate::sdk::SdkPackageInternal::collect_modeled_part_relationships(
    package,
    &mut modeled_part_relationships,
  )?;
  zip.start_file("[Content_Types].xml", options)?;
  zip.write_all(&storage.content_types().to_xml_bytes()?)?;
  let package_relationships = crate::sdk::SdkPackageInternal::modeled_relationships(package)?;
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
    let relationships = modeled_part_relationships
      .remove(&part_id)
      .unwrap_or_default();
    if !relationships.is_empty() {
      let rels_dir_path = crate::common::part_relationships_directory_path(part.path());
      if !rels_dir_path.is_empty() && entry_set.insert(rels_dir_path.clone()) {
        zip.add_directory(&rels_dir_path, options)?;
      }
      let rels_path = crate::common::part_relationships_path(part.path());
      zip.start_file(&rels_path, options)?;
      zip.write_all(&relationships.to_relationships().to_xml_bytes()?)?;
    }
    zip.start_file(part.path(), options)?;
    if let Some(root_element) = package.root_element(part_id) {
      zip.write_all(&root_element.to_xml_bytes()?)?;
    } else {
      zip.write_all(part.data().bytes())?;
    }
  }
  zip.finish()?;
  Ok(())
}
