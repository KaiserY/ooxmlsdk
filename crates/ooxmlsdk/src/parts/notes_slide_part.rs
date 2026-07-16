//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct NotesSlidePart {
    pub(crate) id: crate::common::PartId,
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesSlide,
    >,
    pub(crate) custom_xml_parts: crate::sdk::RepeatedPart<
        crate::parts::custom_xml_part::CustomXmlPart,
    >,
    pub(crate) chart_parts: crate::sdk::RepeatedPart<
        crate::parts::chart_part::ChartPart,
    >,
    pub(crate) extended_chart_parts: crate::sdk::RepeatedPart<
        crate::parts::extended_chart_part::ExtendedChartPart,
    >,
    pub(crate) diagram_colors_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_colors_part::DiagramColorsPart,
    >,
    pub(crate) diagram_data_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_data_part::DiagramDataPart,
    >,
    pub(crate) diagram_persist_layout_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    >,
    pub(crate) diagram_layout_definition_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    >,
    pub(crate) diagram_style_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_style_part::DiagramStylePart,
    >,
    pub(crate) embedded_object_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    pub(crate) embedded_package_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    pub(crate) image_parts: crate::sdk::RepeatedPart<
        crate::parts::image_part::ImagePart,
    >,
    pub(crate) vml_drawing_parts: crate::sdk::RepeatedPart<
        crate::parts::vml_drawing_part::VmlDrawingPart,
    >,
    pub(crate) embedded_control_persistence_binary_data_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    pub(crate) model3_d_reference_relationship_parts: crate::sdk::RepeatedPart<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    pub(crate) notes_master_part: crate::sdk::OptionalPart<
        crate::parts::notes_master_part::NotesMasterPart,
    >,
    pub(crate) theme_override_part: crate::sdk::OptionalPart<
        crate::parts::theme_override_part::ThemeOverridePart,
    >,
    pub(crate) slide_part: crate::sdk::OptionalPart<crate::parts::slide_part::SlidePart>,
    pub(crate) user_defined_tags_parts: crate::sdk::RepeatedPart<
        crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >,
}
