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
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart"
  ))]
  pub(crate) chart_parts: Vec<crate::parts::chart_part::ChartPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx"
  ))]
  pub(crate) extended_chart_parts: Vec<crate::parts::extended_chart_part::ExtendedChartPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors"
  ))]
  pub(crate) diagram_colors_parts: Vec<crate::parts::diagram_colors_part::DiagramColorsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData"
  ))]
  pub(crate) diagram_data_parts: Vec<crate::parts::diagram_data_part::DiagramDataPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing"
  ))]
  pub(crate) diagram_persist_layout_parts:
    Vec<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout"
  ))]
  pub(crate) diagram_layout_definition_parts:
    Vec<crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle"
  ))]
  pub(crate) diagram_style_parts: Vec<crate::parts::diagram_style_part::DiagramStylePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  ))]
  pub(crate) image_parts: Vec<crate::parts::image_part::ImagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"
  ))]
  pub(crate) custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextension"
  ))]
  pub(crate) web_extension_parts: Vec<crate::parts::web_extension_part::WebExtensionPart>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl DrawingsPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
    crate::sdk::PartChildDescriptor::new(
      "chart_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
      "crate::parts::chart_part::ChartPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "extended_chart_parts",
      "http://schemas.microsoft.com/office/2014/relationships/chartEx",
      "crate::parts::extended_chart_part::ExtendedChartPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "diagram_colors_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
      "crate::parts::diagram_colors_part::DiagramColorsPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "diagram_data_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
      "crate::parts::diagram_data_part::DiagramDataPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "diagram_persist_layout_parts",
      "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
      "crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "diagram_layout_definition_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
      "crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "diagram_style_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
      "crate::parts::diagram_style_part::DiagramStylePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "image_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
      "crate::parts::image_part::ImagePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "custom_xml_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
      "crate::parts::custom_xml_part::CustomXmlPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "web_extension_parts",
      "http://schemas.microsoft.com/office/2011/relationships/webextension",
      "crate::parts::web_extension_part::WebExtensionPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
  ];
}
