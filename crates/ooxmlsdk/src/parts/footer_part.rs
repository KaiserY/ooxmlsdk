//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml";
pub const TARGET_NAME: &str = "footer";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct FooterPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_footer_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Footer,
  >,
  #[sdk(part_child(relationship_type = RelationshipAFChunk))]
  pub(crate) alternative_format_import_parts: crate::sdk::RepeatedPart<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
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
  #[sdk(part_child(relationship_type = RelationshipControl))]
  pub(crate) embedded_control_persistence_parts: crate::sdk::RepeatedPart<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
  >,
  #[sdk(part_child(relationship_type = RelationshipOleObject))]
  pub(crate) embedded_object_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  #[sdk(part_child(relationship_type = RelationshipPackage))]
  pub(crate) embedded_package_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(relationship_type = RelationshipImage))]
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  #[sdk(part_child(relationship_type = RelationshipModel3d))]
  pub(crate) model3_d_reference_relationship_parts: crate::sdk::RepeatedPart<
    crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
  >,
}
