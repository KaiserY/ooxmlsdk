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
}
impl DrawingsPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
    DrawingsPart,
    as_drawings_part,
    as_drawings_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated chart_parts => crate ::parts::chart_part::ChartPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart";
      repeated extended_chart_parts => crate
      ::parts::extended_chart_part::ExtendedChartPart,
      "http://schemas.microsoft.com/office/2014/relationships/chartEx"; repeated
      diagram_colors_parts => crate ::parts::diagram_colors_part::DiagramColorsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors";
      repeated diagram_data_parts => crate ::parts::diagram_data_part::DiagramDataPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData";
      repeated diagram_persist_layout_parts => crate
      ::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
      "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing"; repeated
      diagram_layout_definition_parts => crate
      ::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout";
      repeated diagram_style_parts => crate
      ::parts::diagram_style_part::DiagramStylePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle";
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
      repeated custom_xml_parts => crate ::parts::custom_xml_part::CustomXmlPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml";
      repeated web_extension_parts => crate
      ::parts::web_extension_part::WebExtensionPart,
      "http://schemas.microsoft.com/office/2011/relationships/webextension";
  }
}
