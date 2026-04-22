//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument";
pub const PATH_PREFIX: &str = "glossary";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct GlossaryDocumentPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element:
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
  pub wordprocessing_comments_part:
    Option<std::boxed::Box<crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart>>,
  pub document_settings_part:
    Option<std::boxed::Box<crate::parts::document_settings_part::DocumentSettingsPart>>,
  pub endnotes_part: Option<std::boxed::Box<crate::parts::endnotes_part::EndnotesPart>>,
  pub font_table_part: Option<std::boxed::Box<crate::parts::font_table_part::FontTablePart>>,
  pub footnotes_part: Option<std::boxed::Box<crate::parts::footnotes_part::FootnotesPart>>,
  pub numbering_definitions_part:
    Option<std::boxed::Box<crate::parts::numbering_definitions_part::NumberingDefinitionsPart>>,
  pub style_definitions_part:
    Option<std::boxed::Box<crate::parts::style_definitions_part::StyleDefinitionsPart>>,
  #[cfg(feature = "microsoft365")]
  pub styles_with_effects_part:
    Option<std::boxed::Box<crate::parts::styles_with_effects_part::StylesWithEffectsPart>>,
  pub web_settings_part: Option<std::boxed::Box<crate::parts::web_settings_part::WebSettingsPart>>,
  pub footer_parts: Vec<crate::parts::footer_part::FooterPart>,
  pub header_parts: Vec<crate::parts::header_part::HeaderPart>,
  pub wordprocessing_printer_settings_parts:
    Vec<crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart>,
  pub customization_part:
    Option<std::boxed::Box<crate::parts::customization_part::CustomizationPart>>,
  pub vba_project_part: Option<std::boxed::Box<crate::parts::vba_project_part::VbaProjectPart>>,
  #[cfg(feature = "microsoft365")]
  pub wordprocessing_comments_ex_part: Option<
    std::boxed::Box<crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart>,
  >,
  #[cfg(feature = "microsoft365")]
  pub wordprocessing_people_part:
    Option<std::boxed::Box<crate::parts::wordprocessing_people_part::WordprocessingPeoplePart>>,
  #[cfg(feature = "microsoft365")]
  pub wordprocessing_comments_ids_part: Option<
    std::boxed::Box<crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart>,
  >,
  #[cfg(feature = "microsoft365")]
  pub document_tasks_part:
    Option<std::boxed::Box<crate::parts::document_tasks_part::DocumentTasksPart>>,
  #[cfg(feature = "microsoft365")]
  pub word_comments_extensible_part: Option<
    std::boxed::Box<crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart>,
  >,
  pub alternative_format_import_parts:
    Vec<crate::parts::alternative_format_import_part::AlternativeFormatImportPart>,
  pub chart_parts: Vec<crate::parts::chart_part::ChartPart>,
  #[cfg(feature = "microsoft365")]
  pub extended_chart_parts: Vec<crate::parts::extended_chart_part::ExtendedChartPart>,
  pub diagram_colors_parts: Vec<crate::parts::diagram_colors_part::DiagramColorsPart>,
  pub diagram_data_parts: Vec<crate::parts::diagram_data_part::DiagramDataPart>,
  #[cfg(feature = "microsoft365")]
  pub diagram_persist_layout_parts:
    Vec<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  pub diagram_layout_definition_parts:
    Vec<crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart>,
  pub diagram_style_parts: Vec<crate::parts::diagram_style_part::DiagramStylePart>,
  pub embedded_control_persistence_parts:
    Vec<crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart>,
  pub embedded_object_parts: Vec<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  pub embedded_package_parts: Vec<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  pub image_parts: Vec<crate::parts::image_part::ImagePart>,
  pub video_reference_relationships: Vec<crate::common::data_part::VideoReferenceRelationship>,
  #[cfg(feature = "microsoft365")]
  pub model3_d_reference_relationship_parts:
    Vec<crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart>,
}
