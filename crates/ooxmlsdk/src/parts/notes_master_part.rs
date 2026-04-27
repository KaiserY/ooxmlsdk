//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster";
pub const PATH_PREFIX: &str = "notesMasters";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml";
pub const TARGET_NAME: &str = "notesMaster";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct NotesMasterPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl NotesMasterPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesMaster,
    NotesMasterPart,
    as_notes_master_part,
    as_notes_master_part_mut
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
  pub fn slide_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::slide_part::SlidePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::slide_part::SlidePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
    )
  }
}
