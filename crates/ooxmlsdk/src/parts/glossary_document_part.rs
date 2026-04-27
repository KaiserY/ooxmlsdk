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
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[
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
  crate::sdk::PartChildDescriptor::new(
    "wordprocessing_comments_ex_part",
    "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
    "crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "wordprocessing_people_part",
    "http://schemas.microsoft.com/office/2011/relationships/people",
    "crate::parts::wordprocessing_people_part::WordprocessingPeoplePart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "wordprocessing_comments_ids_part",
    "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds",
    "crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "document_tasks_part",
    "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks",
    "crate::parts::document_tasks_part::DocumentTasksPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
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
  crate::sdk::PartChildDescriptor::new(
    "model3_d_reference_relationship_parts",
    "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
    "crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct GlossaryDocumentPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl GlossaryDocumentPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::GlossaryDocument,
    GlossaryDocumentPart,
    as_glossary_document_part,
    as_glossary_document_part_mut
  );
  pub fn wordprocessing_comments_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::wordprocessing_comments_part::WordprocessingCommentsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
    )
  }
  pub fn document_settings_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::document_settings_part::DocumentSettingsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::document_settings_part::DocumentSettingsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings",
    )
  }
  pub fn endnotes_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::endnotes_part::EndnotesPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::endnotes_part::EndnotesPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes",
    )
  }
  pub fn font_table_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::font_table_part::FontTablePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::font_table_part::FontTablePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable",
    )
  }
  pub fn footnotes_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::footnotes_part::FootnotesPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::footnotes_part::FootnotesPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes",
    )
  }
  pub fn numbering_definitions_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::numbering_definitions_part::NumberingDefinitionsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::numbering_definitions_part::NumberingDefinitionsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering",
    )
  }
  pub fn style_definitions_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::style_definitions_part::StyleDefinitionsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::style_definitions_part::StyleDefinitionsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn styles_with_effects_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::styles_with_effects_part::StylesWithEffectsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::styles_with_effects_part::StylesWithEffectsPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2007/relationships/stylesWithEffects",
    )
  }
  pub fn web_settings_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::web_settings_part::WebSettingsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::web_settings_part::WebSettingsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings",
    )
  }
  pub fn footer_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::footer_part::FooterPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::footer_part::FooterPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer",
    )
  }
  pub fn header_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::header_part::HeaderPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::header_part::HeaderPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header",
    )
  }
  pub fn wordprocessing_printer_settings_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<
    Item = crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
  > + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::wordprocessing_printer_settings_part::WordprocessingPrinterSettingsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
    )
  }
  pub fn customization_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::customization_part::CustomizationPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::customization_part::CustomizationPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations",
    )
  }
  pub fn vba_project_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::vba_project_part::VbaProjectPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::vba_project_part::VbaProjectPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn wordprocessing_comments_ex_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::wordprocessing_comments_ex_part::WordprocessingCommentsExPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2011/relationships/commentsExtended",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn wordprocessing_people_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::wordprocessing_people_part::WordprocessingPeoplePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::wordprocessing_people_part::WordprocessingPeoplePart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2011/relationships/people",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn wordprocessing_comments_ids_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::wordprocessing_comments_ids_part::WordprocessingCommentsIdsPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2016/09/relationships/commentsIds",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn document_tasks_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::document_tasks_part::DocumentTasksPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::document_tasks_part::DocumentTasksPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2019/05/relationships/documenttasks",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn word_comments_extensible_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::word_comments_extensible_part::WordCommentsExtensiblePart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2018/08/relationships/commentsExtensible",
    )
  }
  pub fn alternative_format_import_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::alternative_format_import_part::AlternativeFormatImportPart> + 'a
  {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk",
    )
  }
  pub fn chart_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::chart_part::ChartPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::chart_part::ChartPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn extended_chart_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::extended_chart_part::ExtendedChartPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::extended_chart_part::ExtendedChartPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2014/relationships/chartEx",
    )
  }
  pub fn diagram_colors_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::diagram_colors_part::DiagramColorsPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::diagram_colors_part::DiagramColorsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
    )
  }
  pub fn diagram_data_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::diagram_data_part::DiagramDataPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::diagram_data_part::DiagramDataPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn diagram_persist_layout_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart> + 'a
  {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
    )
  }
  pub fn diagram_layout_definition_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart> + 'a
  {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
    )
  }
  pub fn diagram_style_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::diagram_style_part::DiagramStylePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::diagram_style_part::DiagramStylePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
    )
  }
  pub fn embedded_control_persistence_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<
    Item = crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
  > + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
    )
  }
  pub fn embedded_object_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::embedded_object_part::EmbeddedObjectPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::embedded_object_part::EmbeddedObjectPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
    )
  }
  pub fn embedded_package_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::embedded_package_part::EmbeddedPackagePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::embedded_package_part::EmbeddedPackagePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    )
  }
  pub fn image_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::image_part::ImagePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::image_part::ImagePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn model3_d_reference_relationship_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<
    Item = crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
  > + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
    )
  }
}
