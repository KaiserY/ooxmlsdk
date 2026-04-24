//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing";
pub const PATH_PREFIX: &str = "../drawings";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DrawingsPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_drawings_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_spreadsheet_drawing::WorksheetDrawing,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
    kind = "repeated"
  ))]
  pub(crate) chart_parts: crate::sdk::PartChild<crate::parts::chart_part::ChartPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx",
    kind = "repeated"
  ))]
  pub(crate) extended_chart_parts:
    crate::sdk::PartChild<crate::parts::extended_chart_part::ExtendedChartPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
    kind = "repeated"
  ))]
  pub(crate) diagram_colors_parts:
    crate::sdk::PartChild<crate::parts::diagram_colors_part::DiagramColorsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
    kind = "repeated"
  ))]
  pub(crate) diagram_data_parts:
    crate::sdk::PartChild<crate::parts::diagram_data_part::DiagramDataPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
    kind = "repeated"
  ))]
  pub(crate) diagram_persist_layout_parts:
    crate::sdk::PartChild<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
    kind = "repeated"
  ))]
  pub(crate) diagram_layout_definition_parts: crate::sdk::PartChild<
    crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
    kind = "repeated"
  ))]
  pub(crate) diagram_style_parts:
    crate::sdk::PartChild<crate::parts::diagram_style_part::DiagramStylePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    kind = "repeated"
  ))]
  pub(crate) image_parts: crate::sdk::PartChild<crate::parts::image_part::ImagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    kind = "repeated"
  ))]
  pub(crate) custom_xml_parts: crate::sdk::PartChild<crate::parts::custom_xml_part::CustomXmlPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextension",
    kind = "repeated"
  ))]
  pub(crate) web_extension_parts:
    crate::sdk::PartChild<crate::parts::web_extension_part::WebExtensionPart>,
}
