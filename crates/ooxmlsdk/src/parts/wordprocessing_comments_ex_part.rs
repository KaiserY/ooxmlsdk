//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2011/relationships/commentsExtended";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.commentsExtended+xml";
pub const TARGET_NAME: &str = "commentsExtended";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WordprocessingCommentsExPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_wordprocessing_comments_ex_part"))]
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::schemas_microsoft_com_office_word_2012_wordml::CommentsEx>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk"
  ))]
  pub(crate) alternative_format_import_parts:
    Vec<crate::parts::alternative_format_import_part::AlternativeFormatImportPart>,
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
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control"
  ))]
  pub(crate) embedded_control_persistence_parts:
    Vec<crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"
  ))]
  pub(crate) embedded_object_parts: Vec<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"
  ))]
  pub(crate) embedded_package_parts: Vec<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  ))]
  pub(crate) image_parts: Vec<crate::parts::image_part::ImagePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d"
  ))]
  pub(crate) model3_d_reference_relationship_parts:
    Vec<crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl WordprocessingCommentsExPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
    crate::sdk::PartChildDescriptor::new(
      "alternative_format_import_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/aFChunk",
      "crate::parts::alternative_format_import_part::AlternativeFormatImportPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
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
      "embedded_control_persistence_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
      "crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_object_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
      "crate::parts::embedded_object_part::EmbeddedObjectPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "embedded_package_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
      "crate::parts::embedded_package_part::EmbeddedPackagePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    crate::sdk::PartChildDescriptor::new(
      "image_parts",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
      "crate::parts::image_part::ImagePart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "model3_d_reference_relationship_parts",
      "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
      "crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
  ];
}
