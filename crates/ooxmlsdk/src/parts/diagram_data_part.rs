//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData";
pub const PATH_PREFIX: &str = "../graphics";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct DiagramDataPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot,
  pub image_parts: Vec<crate::parts::image_part::ImagePart>,
  pub slide_parts: Vec<crate::parts::slide_part::SlidePart>,
  pub worksheet_parts: Vec<crate::parts::worksheet_part::WorksheetPart>,
}
