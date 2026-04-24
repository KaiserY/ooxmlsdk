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
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
    kind = "optional"
  ))]
  pub(crate) wordprocessing_comments_part:
    crate::sdk::PartChild<crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
    kind = "optional"
  ))]
  pub(crate) document_settings_part:
    crate::sdk::PartChild<crate::parts::document_settings_part::DocumentSettingsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes",
    kind = "optional"
  ))]
  pub(crate) endnotes_part: crate::sdk::PartChild<crate::parts::endnotes_part::EndnotesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
    kind = "optional"
  ))]
  pub(crate) font_table_part: crate::sdk::PartChild<crate::parts::font_table_part::FontTablePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes",
    kind = "optional"
  ))]
  pub(crate) footnotes_part: crate::sdk::PartChild<crate::parts::footnotes_part::FootnotesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
    kind = "optional"
  ))]
  pub(crate) numbering_definitions_part:
    crate::sdk::PartChild<crate::parts::numbering_definitions_part::NumberingDefinitionsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    kind = "optional"
  ))]
  pub(crate) style_definitions_part:
    crate::sdk::PartChild<crate::parts::style_definitions_part::StyleDefinitionsPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects",
    kind = "optional"
  ))]
  pub(crate) styles_with_effects_part:
    crate::sdk::PartChild<crate::parts::styles_with_effects_part::StylesWithEffectsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings",
    kind = "optional"
  ))]
  pub(crate) web_settings_part:
    crate::sdk::PartChild<crate::parts::web_settings_part::WebSettingsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
    kind = "repeated"
  ))]
  pub(crate) footer_parts: crate::sdk::PartChild<crate::parts::footer_part::FooterPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
    kind = "repeated"
  ))]
  pub(crate) header_parts: crate::sdk::PartChild<crate::parts::header_part::HeaderPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
    kind = "repeated"
  ))]
  pub(crate) wordprocessing_printer_settings_parts: crate::sdk::PartChild<
    crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations",
    kind = "optional"
  ))]
  pub(crate) customization_part:
    crate::sdk::PartChild<crate::parts::customization_part::CustomizationPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
    kind = "optional"
  ))]
  pub(crate) vba_project_part:
    crate::sdk::PartChild<crate::parts::vba_project_part::VbaProjectPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
    kind = "optional"
  ))]
  pub(crate) wordprocessing_comments_ex_part: crate::sdk::PartChild<
    crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/people",
    kind = "optional"
  ))]
  pub(crate) wordprocessing_people_part:
    crate::sdk::PartChild<crate::parts::wordprocessing_people_part::WordprocessingPeoplePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds",
    kind = "optional"
  ))]
  pub(crate) wordprocessing_comments_ids_part: crate::sdk::PartChild<
    crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks",
    kind = "optional"
  ))]
  pub(crate) document_tasks_part:
    crate::sdk::PartChild<crate::parts::document_tasks_part::DocumentTasksPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible",
    kind = "optional"
  ))]
  pub(crate) word_comments_extensible_part:
    crate::sdk::PartChild<crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk",
    kind = "repeated"
  ))]
  pub(crate) alternative_format_import_parts: crate::sdk::PartChild<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
    kind = "repeated"
  ))]
  pub(crate) chart_parts: crate::sdk::PartChild<crate::parts::chart_part::ChartPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx",
    kind = "repeated"
  ))]
  pub(crate) extended_chart_parts:
    crate::sdk::PartChild<crate::parts::extended_chart_part::ExtendedChartPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
    kind = "repeated"
  ))]
  pub(crate) diagram_colors_parts:
    crate::sdk::PartChild<crate::parts::diagram_colors_part::DiagramColorsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
    kind = "repeated"
  ))]
  pub(crate) diagram_data_parts:
    crate::sdk::PartChild<crate::parts::diagram_data_part::DiagramDataPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
    kind = "repeated"
  ))]
  pub(crate) diagram_persist_layout_parts:
    crate::sdk::PartChild<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
    kind = "repeated"
  ))]
  pub(crate) diagram_layout_definition_parts: crate::sdk::PartChild<
    crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
    kind = "repeated"
  ))]
  pub(crate) diagram_style_parts:
    crate::sdk::PartChild<crate::parts::diagram_style_part::DiagramStylePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
    kind = "repeated"
  ))]
  pub(crate) embedded_control_persistence_parts: crate::sdk::PartChild<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
    kind = "repeated"
  ))]
  pub(crate) embedded_object_parts:
    crate::sdk::PartChild<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    kind = "repeated"
  ))]
  pub(crate) embedded_package_parts:
    crate::sdk::PartChild<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    kind = "repeated"
  ))]
  pub(crate) image_parts: crate::sdk::PartChild<crate::parts::image_part::ImagePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
    kind = "repeated"
  ))]
  pub(crate) model3_d_reference_relationship_parts: crate::sdk::PartChild<
    crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
  >,
}
