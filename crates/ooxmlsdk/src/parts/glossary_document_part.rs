//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument";
pub const PATH_PREFIX: &str = "glossary";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml";
pub const TARGET_NAME: &str = "document";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct GlossaryDocumentPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_glossary_document_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
  >,
  #[sdk(part_child(relationship_type = RelationshipComments2))]
  pub(crate) wordprocessing_comments_part: crate::sdk::OptionalPart<
    crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipSettings))]
  pub(crate) document_settings_part:
    crate::sdk::OptionalPart<crate::parts::document_settings_part::DocumentSettingsPart>,
  #[sdk(part_child(relationship_type = RelationshipEndnotes))]
  pub(crate) endnotes_part: crate::sdk::OptionalPart<crate::parts::endnotes_part::EndnotesPart>,
  #[sdk(part_child(relationship_type = RelationshipFontTable))]
  pub(crate) font_table_part:
    crate::sdk::OptionalPart<crate::parts::font_table_part::FontTablePart>,
  #[sdk(part_child(relationship_type = RelationshipFootnotes))]
  pub(crate) footnotes_part: crate::sdk::OptionalPart<crate::parts::footnotes_part::FootnotesPart>,
  #[sdk(part_child(relationship_type = RelationshipNumbering))]
  pub(crate) numbering_definitions_part:
    crate::sdk::OptionalPart<crate::parts::numbering_definitions_part::NumberingDefinitionsPart>,
  #[sdk(part_child(relationship_type = RelationshipStyles))]
  pub(crate) style_definitions_part:
    crate::sdk::OptionalPart<crate::parts::style_definitions_part::StyleDefinitionsPart>,
  #[sdk(part_child(relationship_type = RelationshipStylesWithEffects))]
  pub(crate) styles_with_effects_part:
    crate::sdk::OptionalPart<crate::parts::styles_with_effects_part::StylesWithEffectsPart>,
  #[sdk(part_child(relationship_type = RelationshipWebSettings))]
  pub(crate) web_settings_part:
    crate::sdk::OptionalPart<crate::parts::web_settings_part::WebSettingsPart>,
  #[sdk(part_child(relationship_type = RelationshipFooter))]
  pub(crate) footer_parts: crate::sdk::RepeatedPart<crate::parts::footer_part::FooterPart>,
  #[sdk(part_child(relationship_type = RelationshipHeader))]
  pub(crate) header_parts: crate::sdk::RepeatedPart<crate::parts::header_part::HeaderPart>,
  #[sdk(part_child(relationship_type = RelationshipPrinterSettings))]
  pub(crate) wordprocessing_printer_settings_parts: crate::sdk::RepeatedPart<
    crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipKeyMapCustomizations))]
  pub(crate) customization_part:
    crate::sdk::OptionalPart<crate::parts::customization_part::CustomizationPart>,
  #[sdk(part_child(relationship_type = RelationshipVbaProject))]
  pub(crate) vba_project_part:
    crate::sdk::OptionalPart<crate::parts::vba_project_part::VbaProjectPart>,
  #[sdk(part_child(relationship_type = RelationshipCommentsExtended))]
  pub(crate) wordprocessing_comments_ex_part: crate::sdk::OptionalPart<
    crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipPeople))]
  pub(crate) wordprocessing_people_part:
    crate::sdk::OptionalPart<crate::parts::wordprocessing_people_part::WordprocessingPeoplePart>,
  #[sdk(part_child(relationship_type = RelationshipCommentsIds))]
  pub(crate) wordprocessing_comments_ids_part: crate::sdk::OptionalPart<
    crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipDocumenttasks))]
  pub(crate) document_tasks_part:
    crate::sdk::OptionalPart<crate::parts::document_tasks_part::DocumentTasksPart>,
  #[sdk(part_child(relationship_type = RelationshipCommentsExtensible))]
  pub(crate) word_comments_extensible_part: crate::sdk::OptionalPart<
    crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
  >,
  #[sdk(part_child(relationship_type = RelationshipAFChunk))]
  pub(crate) alternative_format_import_parts: crate::sdk::RepeatedPart<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipChart))]
  pub(crate) chart_parts: crate::sdk::RepeatedPart<crate::parts::chart_part::ChartPart>,
  #[sdk(part_child(relationship_type = RelationshipChartEx))]
  pub(crate) extended_chart_parts:
    crate::sdk::RepeatedPart<crate::parts::extended_chart_part::ExtendedChartPart>,
  #[sdk(part_child(relationship_type = RelationshipDiagramColors))]
  pub(crate) diagram_colors_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_colors_part::DiagramColorsPart>,
  #[sdk(part_child(relationship_type = RelationshipDiagramData))]
  pub(crate) diagram_data_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_data_part::DiagramDataPart>,
  #[sdk(part_child(relationship_type = RelationshipDiagramDrawing))]
  pub(crate) diagram_persist_layout_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  #[sdk(part_child(relationship_type = RelationshipDiagramLayout))]
  pub(crate) diagram_layout_definition_parts: crate::sdk::RepeatedPart<
    crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipDiagramQuickStyle))]
  pub(crate) diagram_style_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_style_part::DiagramStylePart>,
  #[sdk(part_child(relationship_type = RelationshipControl))]
  pub(crate) embedded_control_persistence_parts: crate::sdk::RepeatedPart<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
  >,
  #[sdk(part_child(relationship_type = RelationshipOleObject))]
  pub(crate) embedded_object_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(relationship_type = RelationshipPackage))]
  pub(crate) embedded_package_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(relationship_type = RelationshipImage))]
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  #[sdk(part_child(relationship_type = RelationshipModel3d))]
  pub(crate) model3_d_reference_relationship_parts: crate::sdk::RepeatedPart<
    crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
  >,
}
