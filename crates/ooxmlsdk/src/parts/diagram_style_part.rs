//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle";
pub const PATH_PREFIX: &str = "../graphics";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml";
pub const TARGET_NAME: &str = "quickStyle";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DiagramStylePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl DiagramStylePart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::StyleDefinition,
    DiagramStylePart,
    as_diagram_style_part,
    as_diagram_style_part_mut
  );
}
