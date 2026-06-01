//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct GlossaryDocumentPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
  >,
  pub(crate) wordprocessing_comments_part: crate::sdk::OptionalPart<
    crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
  >,
  pub(crate) document_settings_part:
    crate::sdk::OptionalPart<crate::parts::document_settings_part::DocumentSettingsPart>,
  pub(crate) endnotes_part: crate::sdk::OptionalPart<crate::parts::endnotes_part::EndnotesPart>,
  pub(crate) font_table_part:
    crate::sdk::OptionalPart<crate::parts::font_table_part::FontTablePart>,
  pub(crate) footnotes_part: crate::sdk::OptionalPart<crate::parts::footnotes_part::FootnotesPart>,
  pub(crate) numbering_definitions_part:
    crate::sdk::OptionalPart<crate::parts::numbering_definitions_part::NumberingDefinitionsPart>,
  pub(crate) style_definitions_part:
    crate::sdk::OptionalPart<crate::parts::style_definitions_part::StyleDefinitionsPart>,
  pub(crate) styles_with_effects_part:
    crate::sdk::OptionalPart<crate::parts::styles_with_effects_part::StylesWithEffectsPart>,
  pub(crate) web_settings_part:
    crate::sdk::OptionalPart<crate::parts::web_settings_part::WebSettingsPart>,
  pub(crate) footer_parts: crate::sdk::RepeatedPart<crate::parts::footer_part::FooterPart>,
  pub(crate) header_parts: crate::sdk::RepeatedPart<crate::parts::header_part::HeaderPart>,
  pub(crate) wordprocessing_printer_settings_parts: crate::sdk::RepeatedPart<
    crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
  >,
  pub(crate) customization_part:
    crate::sdk::OptionalPart<crate::parts::customization_part::CustomizationPart>,
  pub(crate) vba_project_part:
    crate::sdk::OptionalPart<crate::parts::vba_project_part::VbaProjectPart>,
  pub(crate) wordprocessing_comments_ex_part: crate::sdk::OptionalPart<
    crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
  >,
  pub(crate) wordprocessing_people_part:
    crate::sdk::OptionalPart<crate::parts::wordprocessing_people_part::WordprocessingPeoplePart>,
  pub(crate) wordprocessing_comments_ids_part: crate::sdk::OptionalPart<
    crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
  >,
  pub(crate) document_tasks_part:
    crate::sdk::OptionalPart<crate::parts::document_tasks_part::DocumentTasksPart>,
  pub(crate) word_comments_extensible_part: crate::sdk::OptionalPart<
    crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
  >,
  pub(crate) alternative_format_import_parts: crate::sdk::RepeatedPart<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
  >,
  pub(crate) chart_parts: crate::sdk::RepeatedPart<crate::parts::chart_part::ChartPart>,
  pub(crate) extended_chart_parts:
    crate::sdk::RepeatedPart<crate::parts::extended_chart_part::ExtendedChartPart>,
  pub(crate) diagram_colors_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_colors_part::DiagramColorsPart>,
  pub(crate) diagram_data_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_data_part::DiagramDataPart>,
  pub(crate) diagram_persist_layout_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  pub(crate) diagram_layout_definition_parts: crate::sdk::RepeatedPart<
    crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
  >,
  pub(crate) diagram_style_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_style_part::DiagramStylePart>,
  pub(crate) embedded_control_persistence_parts: crate::sdk::RepeatedPart<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
  >,
  pub(crate) embedded_object_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  pub(crate) embedded_package_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  pub(crate) model3_d_reference_relationship_parts: crate::sdk::RepeatedPart<
    crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
  >,
}
