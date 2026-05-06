//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData";
pub const PATH_PREFIX: &str = "../graphics";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml";
pub const TARGET_NAME: &str = "data";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DiagramDataPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl DiagramDataPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot,
    DiagramDataPart,
    as_diagram_data_part,
    as_diagram_data_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
      repeated slide_parts => crate ::parts::slide_part::SlidePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
      repeated worksheet_parts => crate ::parts::worksheet_part::WorksheetPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
  }
}
