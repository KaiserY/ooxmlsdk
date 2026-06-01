//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct SlidePart {
    pub(crate) relationship_id: Option<String>,
    pub(crate) id: crate::common::PartId,
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide,
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
    pub(crate) slide_comments_part: crate::sdk::OptionalPart<
        crate::parts::slide_comments_part::SlideCommentsPart,
    >,
    pub(crate) notes_slide_part: crate::sdk::OptionalPart<
        crate::parts::notes_slide_part::NotesSlidePart,
    >,
    pub(crate) theme_override_part: crate::sdk::OptionalPart<
        crate::parts::theme_override_part::ThemeOverridePart,
    >,
    pub(crate) slide_layout_part: crate::sdk::OptionalPart<
        crate::parts::slide_layout_part::SlideLayoutPart,
    >,
    pub(crate) slide_sync_data_part: crate::sdk::OptionalPart<
        crate::parts::slide_sync_data_part::SlideSyncDataPart,
    >,
    pub(crate) user_defined_tags_parts: crate::sdk::RepeatedPart<
        crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >,
    pub(crate) slide_parts: crate::sdk::RepeatedPart<
        crate::parts::slide_part::SlidePart,
    >,
    pub(crate) embedded_control_persistence_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    pub(crate) web_extension_parts: crate::sdk::RepeatedPart<
        crate::parts::web_extension_part::WebExtensionPart,
    >,
    pub(crate) comment_parts: crate::sdk::RepeatedPart<
        crate::parts::power_point_comment_part::PowerPointCommentPart,
    >,
}
