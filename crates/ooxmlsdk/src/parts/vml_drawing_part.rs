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
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[
  crate::sdk::PartChildDescriptor::new(
    "image_parts",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    "crate::parts::image_part::ImagePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "legacy_diagram_text_parts",
    "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText",
    "crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct VmlDrawingPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl VmlDrawingPart {
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
  pub fn legacy_diagram_text_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText",
    )
  }
}
