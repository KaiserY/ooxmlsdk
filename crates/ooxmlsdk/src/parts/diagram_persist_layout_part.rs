//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing";
pub const PATH_PREFIX: &str = "../diagrams";
pub const CONTENT_TYPE: &str = "application/vnd.ms-office.drawingml.diagramDrawing+xml";
pub const TARGET_NAME: &str = "drawing";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DiagramPersistLayoutPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl DiagramPersistLayoutPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_drawing_2008_diagram::Drawing,
    DiagramPersistLayoutPart,
    as_diagram_persist_layout_part,
    as_diagram_persist_layout_part_mut
  );
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
}
