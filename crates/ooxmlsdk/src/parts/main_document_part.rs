//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub const PATH_PREFIX: &str = "word";
pub const CONTENT_TYPE: &str = "";
pub const TARGET_NAME: &str = "document";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct MainDocumentPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl MainDocumentPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
    MainDocumentPart,
    as_main_document_part,
    as_main_document_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated custom_xml_parts => crate ::parts::custom_xml_part::CustomXmlPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml";
      optional glossary_document_part => crate
      ::parts::glossary_document_part::GlossaryDocumentPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument";
      optional theme_part => crate ::parts::theme_part::ThemePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
      optional thumbnail_part => crate ::parts::thumbnail_part::ThumbnailPart,
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail";
      optional wordprocessing_comments_part => crate
      ::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments";
      optional document_settings_part => crate
      ::parts::document_settings_part::DocumentSettingsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings";
      optional endnotes_part => crate ::parts::endnotes_part::EndnotesPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes";
      optional font_table_part => crate ::parts::font_table_part::FontTablePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable";
      optional footnotes_part => crate ::parts::footnotes_part::FootnotesPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes";
      optional numbering_definitions_part => crate
      ::parts::numbering_definitions_part::NumberingDefinitionsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering";
      optional style_definitions_part => crate
      ::parts::style_definitions_part::StyleDefinitionsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
      optional styles_with_effects_part => crate
      ::parts::styles_with_effects_part::StylesWithEffectsPart,
      "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects";
      optional web_settings_part => crate ::parts::web_settings_part::WebSettingsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings";
      repeated footer_parts => crate ::parts::footer_part::FooterPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer";
      repeated header_parts => crate ::parts::header_part::HeaderPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header";
      repeated wordprocessing_printer_settings_parts => crate
      ::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings";
      optional customization_part => crate
      ::parts::customization_part::CustomizationPart,
      "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations";
      optional vba_project_part => crate ::parts::vba_project_part::VbaProjectPart,
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject"; optional
      wordprocessing_comments_ex_part => crate
      ::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
      "http://schemas.microsoft.com/office/2011/relationships/commentsExtended";
      optional wordprocessing_people_part => crate
      ::parts::wordprocessing_people_part::WordprocessingPeoplePart,
      "http://schemas.microsoft.com/office/2011/relationships/people"; optional
      wordprocessing_comments_ids_part => crate
      ::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
      "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds"; optional
      document_tasks_part => crate ::parts::document_tasks_part::DocumentTasksPart,
      "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks";
      optional word_comments_extensible_part => crate
      ::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
      "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible";
      repeated alternative_format_import_parts => crate
      ::parts::alternative_format_import_part::AlternativeFormatImportPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk";
      repeated chart_parts => crate ::parts::chart_part::ChartPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart";
      repeated extended_chart_parts => crate
      ::parts::extended_chart_part::ExtendedChartPart,
      "http://schemas.microsoft.com/office/2014/relationships/chartEx"; repeated
      diagram_colors_parts => crate ::parts::diagram_colors_part::DiagramColorsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors";
      repeated diagram_data_parts => crate ::parts::diagram_data_part::DiagramDataPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData";
      repeated diagram_persist_layout_parts => crate
      ::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
      "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing"; repeated
      diagram_layout_definition_parts => crate
      ::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout";
      repeated diagram_style_parts => crate
      ::parts::diagram_style_part::DiagramStylePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle";
      repeated embedded_control_persistence_parts => crate
      ::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control";
      repeated embedded_object_parts => crate
      ::parts::embedded_object_part::EmbeddedObjectPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject";
      repeated embedded_package_parts => crate
      ::parts::embedded_package_part::EmbeddedPackagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package";
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
      repeated model3_d_reference_relationship_parts => crate
      ::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d";
  }
}
