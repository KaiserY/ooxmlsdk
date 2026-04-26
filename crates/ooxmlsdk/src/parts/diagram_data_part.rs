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
  #[sdk(part_root(accessor = "as_diagram_data_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  ))]
  pub(crate) image_parts: Vec<crate::parts::image_part::ImagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide"
  ))]
  pub(crate) slide_parts: Vec<crate::parts::slide_part::SlidePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet"
  ))]
  pub(crate) worksheet_parts: Vec<crate::parts::worksheet_part::WorksheetPart>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl DiagramDataPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
    crate::sdk::PartChildDescriptor::new(
      "image_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
      "crate::parts::image_part::ImagePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "slide_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
      "crate::parts::slide_part::SlidePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "worksheet_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet",
      "crate::parts::worksheet_part::WorksheetPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
  ];
}
