//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument";
pub const PATH_PREFIX: &str = "glossary";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct GlossaryDocumentPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_glossary_document_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments"
  ))]
  pub(crate) wordprocessing_comments_part: crate::sdk::OptionalPart<
    crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings"
  ))]
  pub(crate) document_settings_part:
    crate::sdk::OptionalPart<crate::parts::document_settings_part::DocumentSettingsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes"
  ))]
  pub(crate) endnotes_part: crate::sdk::OptionalPart<crate::parts::endnotes_part::EndnotesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable"
  ))]
  pub(crate) font_table_part:
    crate::sdk::OptionalPart<crate::parts::font_table_part::FontTablePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes"
  ))]
  pub(crate) footnotes_part: crate::sdk::OptionalPart<crate::parts::footnotes_part::FootnotesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering"
  ))]
  pub(crate) numbering_definitions_part:
    crate::sdk::OptionalPart<crate::parts::numbering_definitions_part::NumberingDefinitionsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles"
  ))]
  pub(crate) style_definitions_part:
    crate::sdk::OptionalPart<crate::parts::style_definitions_part::StyleDefinitionsPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects"
  ))]
  pub(crate) styles_with_effects_part:
    crate::sdk::OptionalPart<crate::parts::styles_with_effects_part::StylesWithEffectsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings"
  ))]
  pub(crate) web_settings_part:
    crate::sdk::OptionalPart<crate::parts::web_settings_part::WebSettingsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer"
  ))]
  pub(crate) footer_parts: crate::sdk::RepeatedPart<crate::parts::footer_part::FooterPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header"
  ))]
  pub(crate) header_parts: crate::sdk::RepeatedPart<crate::parts::header_part::HeaderPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings"
  ))]
  pub(crate) wordprocessing_printer_settings_parts: crate::sdk::RepeatedPart<
    crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations"
  ))]
  pub(crate) customization_part:
    crate::sdk::OptionalPart<crate::parts::customization_part::CustomizationPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject"
  ))]
  pub(crate) vba_project_part:
    crate::sdk::OptionalPart<crate::parts::vba_project_part::VbaProjectPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/commentsExtended"
  ))]
  pub(crate) wordprocessing_comments_ex_part: crate::sdk::OptionalPart<
    crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/people"
  ))]
  pub(crate) wordprocessing_people_part:
    crate::sdk::OptionalPart<crate::parts::wordprocessing_people_part::WordprocessingPeoplePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds"
  ))]
  pub(crate) wordprocessing_comments_ids_part: crate::sdk::OptionalPart<
    crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks"
  ))]
  pub(crate) document_tasks_part:
    crate::sdk::OptionalPart<crate::parts::document_tasks_part::DocumentTasksPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible"
  ))]
  pub(crate) word_comments_extensible_part: crate::sdk::OptionalPart<
    crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk"
  ))]
  pub(crate) alternative_format_import_parts: crate::sdk::RepeatedPart<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart"
  ))]
  pub(crate) chart_parts: crate::sdk::RepeatedPart<crate::parts::chart_part::ChartPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx"
  ))]
  pub(crate) extended_chart_parts:
    crate::sdk::RepeatedPart<crate::parts::extended_chart_part::ExtendedChartPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors"
  ))]
  pub(crate) diagram_colors_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_colors_part::DiagramColorsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData"
  ))]
  pub(crate) diagram_data_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_data_part::DiagramDataPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing"
  ))]
  pub(crate) diagram_persist_layout_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout"
  ))]
  pub(crate) diagram_layout_definition_parts: crate::sdk::RepeatedPart<
    crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle"
  ))]
  pub(crate) diagram_style_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_style_part::DiagramStylePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control"
  ))]
  pub(crate) embedded_control_persistence_parts: crate::sdk::RepeatedPart<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"
  ))]
  pub(crate) embedded_object_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"
  ))]
  pub(crate) embedded_package_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  ))]
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d"
  ))]
  pub(crate) model3_d_reference_relationship_parts: crate::sdk::RepeatedPart<
    crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
  >,
}
