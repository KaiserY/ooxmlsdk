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
  #[sdk(part_root(accessor = "as_main_document_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Document,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"
  ))]
  pub(crate) custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument"
  ))]
  pub(crate) glossary_document_part:
    Option<Box<crate::parts::glossary_document_part::GlossaryDocumentPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
  ))]
  pub(crate) theme_part: Option<Box<crate::parts::theme_part::ThemePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
  ))]
  pub(crate) thumbnail_part: Option<Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments"
  ))]
  pub(crate) wordprocessing_comments_part:
    Option<Box<crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings"
  ))]
  pub(crate) document_settings_part:
    Option<Box<crate::parts::document_settings_part::DocumentSettingsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes"
  ))]
  pub(crate) endnotes_part: Option<Box<crate::parts::endnotes_part::EndnotesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable"
  ))]
  pub(crate) font_table_part: Option<Box<crate::parts::font_table_part::FontTablePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes"
  ))]
  pub(crate) footnotes_part: Option<Box<crate::parts::footnotes_part::FootnotesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering"
  ))]
  pub(crate) numbering_definitions_part:
    Option<Box<crate::parts::numbering_definitions_part::NumberingDefinitionsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles"
  ))]
  pub(crate) style_definitions_part:
    Option<Box<crate::parts::style_definitions_part::StyleDefinitionsPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects"
  ))]
  pub(crate) styles_with_effects_part:
    Option<Box<crate::parts::styles_with_effects_part::StylesWithEffectsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings"
  ))]
  pub(crate) web_settings_part: Option<Box<crate::parts::web_settings_part::WebSettingsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer"
  ))]
  pub(crate) footer_parts: Vec<crate::parts::footer_part::FooterPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header"
  ))]
  pub(crate) header_parts: Vec<crate::parts::header_part::HeaderPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings"
  ))]
  pub(crate) wordprocessing_printer_settings_parts:
    Vec<crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations"
  ))]
  pub(crate) customization_part: Option<Box<crate::parts::customization_part::CustomizationPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject"
  ))]
  pub(crate) vba_project_part: Option<Box<crate::parts::vba_project_part::VbaProjectPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/commentsExtended"
  ))]
  pub(crate) wordprocessing_comments_ex_part:
    Option<Box<crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/people"
  ))]
  pub(crate) wordprocessing_people_part:
    Option<Box<crate::parts::wordprocessing_people_part::WordprocessingPeoplePart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds"
  ))]
  pub(crate) wordprocessing_comments_ids_part:
    Option<Box<crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks"
  ))]
  pub(crate) document_tasks_part: Option<Box<crate::parts::document_tasks_part::DocumentTasksPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible"
  ))]
  pub(crate) word_comments_extensible_part:
    Option<Box<crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk"
  ))]
  pub(crate) alternative_format_import_parts:
    Vec<crate::parts::alternative_format_import_part::AlternativeFormatImportPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart"
  ))]
  pub(crate) chart_parts: Vec<crate::parts::chart_part::ChartPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx"
  ))]
  pub(crate) extended_chart_parts: Vec<crate::parts::extended_chart_part::ExtendedChartPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors"
  ))]
  pub(crate) diagram_colors_parts: Vec<crate::parts::diagram_colors_part::DiagramColorsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData"
  ))]
  pub(crate) diagram_data_parts: Vec<crate::parts::diagram_data_part::DiagramDataPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing"
  ))]
  pub(crate) diagram_persist_layout_parts:
    Vec<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout"
  ))]
  pub(crate) diagram_layout_definition_parts:
    Vec<crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle"
  ))]
  pub(crate) diagram_style_parts: Vec<crate::parts::diagram_style_part::DiagramStylePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control"
  ))]
  pub(crate) embedded_control_persistence_parts:
    Vec<crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"
  ))]
  pub(crate) embedded_object_parts: Vec<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"
  ))]
  pub(crate) embedded_package_parts: Vec<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  ))]
  pub(crate) image_parts: Vec<crate::parts::image_part::ImagePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d"
  ))]
  pub(crate) model3_d_reference_relationship_parts:
    Vec<crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl MainDocumentPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
    crate::sdk::PartChildDescriptor::new(
      "custom_xml_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
      "crate::parts::custom_xml_part::CustomXmlPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "glossary_document_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument",
      "crate::parts::glossary_document_part::GlossaryDocumentPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "theme_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
      "crate::parts::theme_part::ThemePart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "thumbnail_part",
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
      "crate::parts::thumbnail_part::ThumbnailPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "wordprocessing_comments_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
      "crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "document_settings_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
      "crate::parts::document_settings_part::DocumentSettingsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "endnotes_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes",
      "crate::parts::endnotes_part::EndnotesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "font_table_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
      "crate::parts::font_table_part::FontTablePart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "footnotes_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes",
      "crate::parts::footnotes_part::FootnotesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "numbering_definitions_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
      "crate::parts::numbering_definitions_part::NumberingDefinitionsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "style_definitions_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
      "crate::parts::style_definitions_part::StyleDefinitionsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "styles_with_effects_part",
      "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects",
      "crate::parts::styles_with_effects_part::StylesWithEffectsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "web_settings_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings",
      "crate::parts::web_settings_part::WebSettingsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "footer_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
      "crate::parts::footer_part::FooterPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "header_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
      "crate::parts::header_part::HeaderPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "wordprocessing_printer_settings_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
      "crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "customization_part",
      "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations",
      "crate::parts::customization_part::CustomizationPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "vba_project_part",
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
      "crate::parts::vba_project_part::VbaProjectPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "wordprocessing_comments_ex_part",
      "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
      "crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "wordprocessing_people_part",
      "http://schemas.microsoft.com/office/2011/relationships/people",
      "crate::parts::wordprocessing_people_part::WordprocessingPeoplePart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "wordprocessing_comments_ids_part",
      "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds",
      "crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "document_tasks_part",
      "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks",
      "crate::parts::document_tasks_part::DocumentTasksPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "word_comments_extensible_part",
      "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible",
      "crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "alternative_format_import_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk",
      "crate::parts::alternative_format_import_part::AlternativeFormatImportPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "chart_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
      "crate::parts::chart_part::ChartPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "extended_chart_parts",
      "http://schemas.microsoft.com/office/2014/relationships/chartEx",
      "crate::parts::extended_chart_part::ExtendedChartPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "diagram_colors_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
      "crate::parts::diagram_colors_part::DiagramColorsPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "diagram_data_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
      "crate::parts::diagram_data_part::DiagramDataPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "diagram_persist_layout_parts",
      "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
      "crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "diagram_layout_definition_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
      "crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "diagram_style_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
      "crate::parts::diagram_style_part::DiagramStylePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_control_persistence_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
      "crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_object_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
      "crate::parts::embedded_object_part::EmbeddedObjectPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_package_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
      "crate::parts::embedded_package_part::EmbeddedPackagePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "image_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
      "crate::parts::image_part::ImagePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "model3_d_reference_relationship_parts",
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
      "crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
  ];
}
