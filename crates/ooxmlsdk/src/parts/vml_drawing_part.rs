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
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  ))]
  pub(crate) image_parts: Vec<crate::parts::image_part::ImagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/legacyDiagramText"
  ))]
  pub(crate) legacy_diagram_text_parts:
    Vec<crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl VmlDrawingPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
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
}
