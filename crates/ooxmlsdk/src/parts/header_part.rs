//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml";
pub const TARGET_NAME: &str = "header";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct HeaderPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_header_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Header,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk",
    kind = "repeated"
  ))]
  pub(crate) alternative_format_import_parts:
    Vec<crate::parts::alternative_format_import_part::AlternativeFormatImportPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
    kind = "repeated"
  ))]
  pub(crate) chart_parts: Vec<crate::parts::chart_part::ChartPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx",
    kind = "repeated"
  ))]
  pub(crate) extended_chart_parts: Vec<crate::parts::extended_chart_part::ExtendedChartPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
    kind = "repeated"
  ))]
  pub(crate) diagram_colors_parts: Vec<crate::parts::diagram_colors_part::DiagramColorsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
    kind = "repeated"
  ))]
  pub(crate) diagram_data_parts: Vec<crate::parts::diagram_data_part::DiagramDataPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
    kind = "repeated"
  ))]
  pub(crate) diagram_persist_layout_parts:
    Vec<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
    kind = "repeated"
  ))]
  pub(crate) diagram_layout_definition_parts:
    Vec<crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
    kind = "repeated"
  ))]
  pub(crate) diagram_style_parts: Vec<crate::parts::diagram_style_part::DiagramStylePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
    kind = "repeated"
  ))]
  pub(crate) embedded_control_persistence_parts:
    Vec<crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
    kind = "repeated"
  ))]
  pub(crate) embedded_object_parts: Vec<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    kind = "repeated"
  ))]
  pub(crate) embedded_package_parts: Vec<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    kind = "repeated"
  ))]
  pub(crate) image_parts: Vec<crate::parts::image_part::ImagePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
    kind = "repeated"
  ))]
  pub(crate) model3_d_reference_relationship_parts:
    Vec<crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
