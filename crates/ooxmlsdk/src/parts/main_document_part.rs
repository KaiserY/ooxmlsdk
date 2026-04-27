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
  pub fn custom_xml_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::custom_xml_part::CustomXmlPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::custom_xml_part::CustomXmlPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    )
  }
  pub fn glossary_document_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::glossary_document_part::GlossaryDocumentPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::glossary_document_part::GlossaryDocumentPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/glossaryDocument",
    )
  }
  pub fn theme_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::theme_part::ThemePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::theme_part::ThemePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    )
  }
  pub fn thumbnail_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::thumbnail_part::ThumbnailPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::thumbnail_part::ThumbnailPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    )
  }
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
