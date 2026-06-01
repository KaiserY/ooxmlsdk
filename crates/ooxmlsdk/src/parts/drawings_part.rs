//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing";
pub const PATH_PREFIX: &str = "../drawings";
pub const CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawing+xml";
pub const TARGET_NAME: &str = "drawing";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DrawingsPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_drawings_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
  >,
  #[sdk(part_child(relationship_type = RelationshipChart))]
  pub(crate) chart_parts: crate::sdk::RepeatedPart<crate::parts::chart_part::ChartPart>,
  #[sdk(part_child(relationship_type = RelationshipChartEx))]
  pub(crate) extended_chart_parts:
    crate::sdk::RepeatedPart<crate::parts::extended_chart_part::ExtendedChartPart>,
  #[sdk(part_child(relationship_type = RelationshipDiagramColors))]
  pub(crate) diagram_colors_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_colors_part::DiagramColorsPart>,
  #[sdk(part_child(relationship_type = RelationshipDiagramData))]
  pub(crate) diagram_data_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_data_part::DiagramDataPart>,
  #[sdk(part_child(relationship_type = RelationshipDiagramDrawing))]
  pub(crate) diagram_persist_layout_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  #[sdk(part_child(relationship_type = RelationshipDiagramLayout))]
  pub(crate) diagram_layout_definition_parts: crate::sdk::RepeatedPart<
    crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipDiagramQuickStyle))]
  pub(crate) diagram_style_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_style_part::DiagramStylePart>,
  #[sdk(part_child(relationship_type = RelationshipImage))]
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  #[sdk(part_child(relationship_type = RelationshipCustomXml))]
  pub(crate) custom_xml_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(relationship_type = RelationshipWebextension))]
  pub(crate) web_extension_parts:
    crate::sdk::RepeatedPart<crate::parts::web_extension_part::WebExtensionPart>,
}
