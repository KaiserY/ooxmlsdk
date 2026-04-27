//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
pub const PATH_PREFIX: &str = "slides";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
pub const TARGET_NAME: &str = "slide";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[
  crate::sdk::PartChildDescriptor::new(
    "custom_xml_parts",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    "crate::parts::custom_xml_part::CustomXmlPart",
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
    "vml_drawing_parts",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
    "crate::parts::vml_drawing_part::VmlDrawingPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "embedded_control_persistence_binary_data_parts",
    "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
    "crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "model3_d_reference_relationship_parts",
    "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
    "crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "slide_comments_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
    "crate::parts::slide_comments_part::SlideCommentsPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "notes_slide_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide",
    "crate::parts::notes_slide_part::NotesSlidePart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "theme_override_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride",
    "crate::parts::theme_override_part::ThemeOverridePart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "slide_layout_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout",
    "crate::parts::slide_layout_part::SlideLayoutPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "slide_sync_data_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo",
    "crate::parts::slide_sync_data_part::SlideSyncDataPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "user_defined_tags_parts",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags",
    "crate::parts::user_defined_tags_part::UserDefinedTagsPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "slide_parts",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
    "crate::parts::slide_part::SlidePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "embedded_control_persistence_parts",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
    "crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "web_extension_parts",
    "http://schemas.microsoft.com/office/2011/relationships/webextension",
    "crate::parts::web_extension_part::WebExtensionPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "comment_parts",
    "http://schemas.microsoft.com/office/2018/10/relationships/comments",
    "crate::parts::power_point_comment_part::PowerPointCommentPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct SlidePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl SlidePart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide,
    SlidePart,
    as_slide_part,
    as_slide_part_mut
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
  pub fn vml_drawing_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::vml_drawing_part::VmlDrawingPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::vml_drawing_part::VmlDrawingPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
    )
  }
    pub fn embedded_control_persistence_binary_data_parts<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
    ) -> impl Iterator<
        Item = crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
  > + 'a{
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
            P,
            crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
        >(
            self,
            package,
            "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
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
  pub fn slide_comments_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::slide_comments_part::SlideCommentsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::slide_comments_part::SlideCommentsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
    )
  }
  pub fn notes_slide_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::notes_slide_part::NotesSlidePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::notes_slide_part::NotesSlidePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide",
    )
  }
  pub fn theme_override_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::theme_override_part::ThemeOverridePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::theme_override_part::ThemeOverridePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride",
    )
  }
  pub fn slide_layout_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::slide_layout_part::SlideLayoutPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::slide_layout_part::SlideLayoutPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout",
    )
  }
  pub fn slide_sync_data_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::slide_sync_data_part::SlideSyncDataPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::slide_sync_data_part::SlideSyncDataPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo",
    )
  }
  pub fn user_defined_tags_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::user_defined_tags_part::UserDefinedTagsPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags",
    )
  }
  pub fn slide_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::slide_part::SlidePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::slide_part::SlidePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
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
  #[cfg(feature = "microsoft365")]
  pub fn web_extension_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::web_extension_part::WebExtensionPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::web_extension_part::WebExtensionPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2011/relationships/webextension",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn comment_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::power_point_comment_part::PowerPointCommentPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::power_point_comment_part::PowerPointCommentPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2018/10/relationships/comments",
    )
  }
}
