//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WordprocessingCommentsPart {
  pub(crate) id: crate::common::PartId,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Comments,
  >,
  pub(crate) alternative_format_import_parts: crate::sdk::RepeatedPart<
    crate::parts::alternative_format_import_part::AlternativeFormatImportPart,
  >,
  pub(crate) chart_parts: crate::sdk::RepeatedPart<crate::parts::chart_part::ChartPart>,
  pub(crate) extended_chart_parts:
    crate::sdk::RepeatedPart<crate::parts::extended_chart_part::ExtendedChartPart>,
  pub(crate) diagram_colors_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_colors_part::DiagramColorsPart>,
  pub(crate) diagram_data_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_data_part::DiagramDataPart>,
  pub(crate) diagram_persist_layout_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart>,
  pub(crate) diagram_layout_definition_parts: crate::sdk::RepeatedPart<
    crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
  >,
  pub(crate) diagram_style_parts:
    crate::sdk::RepeatedPart<crate::parts::diagram_style_part::DiagramStylePart>,
  pub(crate) embedded_control_persistence_parts: crate::sdk::RepeatedPart<
    crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
  >,
  pub(crate) embedded_object_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_object_part::EmbeddedObjectPart>,
  pub(crate) embedded_package_parts:
    crate::sdk::RepeatedPart<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  pub(crate) model3_d_reference_relationship_parts: crate::sdk::RepeatedPart<
    crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
  >,
}
