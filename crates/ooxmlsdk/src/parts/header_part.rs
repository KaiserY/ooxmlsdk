//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml";
pub const TARGET_NAME: &str = "header";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct HeaderPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl HeaderPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Header,
    HeaderPart,
    as_header_part,
    as_header_part_mut
  );
  crate::sdk_part_child_methods! {
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
