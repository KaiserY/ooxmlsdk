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
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_relationships)]
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  #[sdk(part_rels_path)]
  pub rels_path: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element:
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
    kind = "optional"
  ))]
  pub wordprocessing_comments_part:
    Option<std::boxed::Box<crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
    kind = "optional"
  ))]
  pub document_settings_part:
    Option<std::boxed::Box<crate::parts::document_settings_part::DocumentSettingsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes",
    kind = "optional"
  ))]
  pub endnotes_part: Option<std::boxed::Box<crate::parts::endnotes_part::EndnotesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
    kind = "optional"
  ))]
  pub font_table_part: Option<std::boxed::Box<crate::parts::font_table_part::FontTablePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes",
    kind = "optional"
  ))]
  pub footnotes_part: Option<std::boxed::Box<crate::parts::footnotes_part::FootnotesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
    kind = "optional"
  ))]
  pub numbering_definitions_part:
    Option<std::boxed::Box<crate::parts::numbering_definitions_part::NumberingDefinitionsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    kind = "optional"
  ))]
  pub style_definitions_part:
    Option<std::boxed::Box<crate::parts::style_definitions_part::StyleDefinitionsPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects",
    kind = "optional"
  ))]
  pub styles_with_effects_part:
    Option<std::boxed::Box<crate::parts::styles_with_effects_part::StylesWithEffectsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings",
    kind = "optional"
  ))]
  pub web_settings_part: Option<std::boxed::Box<crate::parts::web_settings_part::WebSettingsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
    kind = "repeated"
  ))]
  pub footer_parts: Vec<crate::parts::footer_part::FooterPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
    kind = "repeated"
  ))]
  pub header_parts: Vec<crate::parts::header_part::HeaderPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
    kind = "repeated"
  ))]
  pub wordprocessing_printer_settings_parts:
    Vec<crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations",
    kind = "optional"
  ))]
  pub customization_part:
    Option<std::boxed::Box<crate::parts::customization_part::CustomizationPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
    kind = "optional"
  ))]
  pub vba_project_part: Option<std::boxed::Box<crate::parts::vba_project_part::VbaProjectPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
    kind = "optional"
  ))]
  pub wordprocessing_comments_ex_part: Option<
    std::boxed::Box<crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart>,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/people",
    kind = "optional"
  ))]
  pub wordprocessing_people_part:
    Option<std::boxed::Box<crate::parts::wordprocessing_people_part::WordprocessingPeoplePart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds",
    kind = "optional"
  ))]
  pub wordprocessing_comments_ids_part: Option<
    std::boxed::Box<crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart>,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks",
    kind = "optional"
  ))]
  pub document_tasks_part:
    Option<std::boxed::Box<crate::parts::document_tasks_part::DocumentTasksPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible",
    kind = "optional"
  ))]
  pub word_comments_extensible_part: Option<
    std::boxed::Box<crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart>,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk",
    kind = "repeated"
  ))]
  pub alternative_format_import_parts:
    Vec<crate::parts::alternative_format_import_part::AlternativeFormatImportPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
    kind = "repeated"
  ))]
  pub chart_parts: Vec<crate::parts::chart_part::ChartPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx",
    kind = "repeated"
  ))]
  pub extended_chart_parts: Vec<crate::parts::extended_chart_part::ExtendedChartPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
    kind = "repeated"
  ))]
  pub diagram_colors_parts: Vec<crate::parts::diagram_colors_part::DiagramColorsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
    kind = "repeated"
  ))]
  pub diagram_data_parts: Vec<crate::parts::diagram_data_part::DiagramDataPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
    kind = "repeated"
  ))]
  pub diagram_persist_layout_parts:
    Vec<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
    kind = "repeated"
  ))]
  pub diagram_layout_definition_parts:
    Vec<crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
    kind = "repeated"
  ))]
  pub diagram_style_parts: Vec<crate::parts::diagram_style_part::DiagramStylePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
    kind = "repeated"
  ))]
  pub embedded_control_persistence_parts:
    Vec<crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
    kind = "repeated"
  ))]
  pub embedded_object_parts: Vec<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    kind = "repeated"
  ))]
  pub embedded_package_parts: Vec<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    kind = "repeated"
  ))]
  pub image_parts: Vec<crate::parts::image_part::ImagePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
    kind = "repeated"
  ))]
  pub model3_d_reference_relationship_parts:
    Vec<crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart>,
}
