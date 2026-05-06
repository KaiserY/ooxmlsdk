//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing";
pub const PATH_PREFIX: &str = "../drawings";
pub const CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.vmlDrawing";
pub const TARGET_NAME: &str = "vmldrawing";
pub const EXTENSION: &str = ".vml";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct VmlDrawingPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl VmlDrawingPart {
  crate::sdk_part_child_methods! {
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
      repeated legacy_diagram_text_parts => crate
      ::parts::legacy_diagram_text_part::LegacyDiagramTextPart,
      "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText";
  }
}
