//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout";
pub const PATH_PREFIX: &str = "../slideLayouts";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml";
pub const TARGET_NAME: &str = "slideLayout";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct SlideLayoutPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl SlideLayoutPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideLayout,
    SlideLayoutPart,
    as_slide_layout_part,
    as_slide_layout_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated custom_xml_parts => crate ::parts::custom_xml_part::CustomXmlPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml";
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
      repeated embedded_object_parts => crate
      ::parts::embedded_object_part::EmbeddedObjectPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject";
      repeated embedded_package_parts => crate
      ::parts::embedded_package_part::EmbeddedPackagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package";
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
      repeated vml_drawing_parts => crate ::parts::vml_drawing_part::VmlDrawingPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing";
      repeated embedded_control_persistence_binary_data_parts => crate
      ::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
      "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary";
      repeated model3_d_reference_relationship_parts => crate
      ::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d"; repeated
      slide_parts => crate ::parts::slide_part::SlidePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
      optional slide_master_part => crate ::parts::slide_master_part::SlideMasterPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";
      optional theme_override_part => crate
      ::parts::theme_override_part::ThemeOverridePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride";
      repeated user_defined_tags_parts => crate
      ::parts::user_defined_tags_part::UserDefinedTagsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags";
      repeated embedded_control_persistence_parts => crate
      ::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control";
  }
}
