//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout";
pub const PATH_PREFIX: &str = "../graphics";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DiagramLayoutDefinitionPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_diagram_layout_definition_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::LayoutDefinition,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  ))]
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
