//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing";
pub const PATH_PREFIX: &str = "../drawings";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct DrawingsPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element:
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
  pub chart_parts: Vec<crate::parts::chart_part::ChartPart>,
  #[cfg(feature = "microsoft365")]
  pub extended_chart_parts: Vec<crate::parts::extended_chart_part::ExtendedChartPart>,
  pub diagram_colors_parts: Vec<crate::parts::diagram_colors_part::DiagramColorsPart>,
  pub diagram_data_parts: Vec<crate::parts::diagram_data_part::DiagramDataPart>,
  #[cfg(feature = "microsoft365")]
  pub diagram_persist_layout_parts:
    Vec<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  pub diagram_layout_definition_parts:
    Vec<crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart>,
  pub diagram_style_parts: Vec<crate::parts::diagram_style_part::DiagramStylePart>,
  pub image_parts: Vec<crate::parts::image_part::ImagePart>,
  pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  #[cfg(feature = "microsoft365")]
  pub web_extension_parts: Vec<crate::parts::web_extension_part::WebExtensionPart>,
}
