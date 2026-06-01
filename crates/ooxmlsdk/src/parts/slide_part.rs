//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
pub const PATH_PREFIX: &str = "slides";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.presentationml.slide+xml";
pub const TARGET_NAME: &str = "slide";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct SlidePart {
    pub(crate) relationship_id: Option<String>,
    pub(crate) id: crate::common::PartId,
    #[sdk(part_root(accessor = "as_slide_part"))]
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide,
    >,
    #[sdk(part_child(relationship_type = RelationshipCustomXml))]
    pub(crate) custom_xml_parts: crate::sdk::RepeatedPart<
        crate::parts::custom_xml_part::CustomXmlPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipChart))]
    pub(crate) chart_parts: crate::sdk::RepeatedPart<
        crate::parts::chart_part::ChartPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipChartEx))]
    pub(crate) extended_chart_parts: crate::sdk::RepeatedPart<
        crate::parts::extended_chart_part::ExtendedChartPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipDiagramColors))]
    pub(crate) diagram_colors_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_colors_part::DiagramColorsPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipDiagramData))]
    pub(crate) diagram_data_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_data_part::DiagramDataPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipDiagramDrawing))]
    pub(crate) diagram_persist_layout_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipDiagramLayout))]
    pub(crate) diagram_layout_definition_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipDiagramQuickStyle))]
    pub(crate) diagram_style_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_style_part::DiagramStylePart,
    >,
    #[sdk(part_child(relationship_type = RelationshipOleObject))]
    pub(crate) embedded_object_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipPackage))]
    pub(crate) embedded_package_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    #[sdk(part_child(relationship_type = RelationshipImage))]
    pub(crate) image_parts: crate::sdk::RepeatedPart<
        crate::parts::image_part::ImagePart,
    >,
    #[sdk(part_child(relationship_type = RelationshipVmlDrawing))]
    pub(crate) vml_drawing_parts: crate::sdk::RepeatedPart<
        crate::parts::vml_drawing_part::VmlDrawingPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipActiveXControlBinary))]
    pub(crate) embedded_control_persistence_binary_data_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipModel3d))]
    pub(crate) model3_d_reference_relationship_parts: crate::sdk::RepeatedPart<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipComments2))]
    pub(crate) slide_comments_part: crate::sdk::OptionalPart<
        crate::parts::slide_comments_part::SlideCommentsPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipNotesSlide))]
    pub(crate) notes_slide_part: crate::sdk::OptionalPart<
        crate::parts::notes_slide_part::NotesSlidePart,
    >,
    #[sdk(part_child(relationship_type = RelationshipThemeOverride))]
    pub(crate) theme_override_part: crate::sdk::OptionalPart<
        crate::parts::theme_override_part::ThemeOverridePart,
    >,
    #[sdk(part_child(relationship_type = RelationshipSlideLayout))]
    pub(crate) slide_layout_part: crate::sdk::OptionalPart<
        crate::parts::slide_layout_part::SlideLayoutPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipSlideUpdateInfo))]
    pub(crate) slide_sync_data_part: crate::sdk::OptionalPart<
        crate::parts::slide_sync_data_part::SlideSyncDataPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipTags))]
    pub(crate) user_defined_tags_parts: crate::sdk::RepeatedPart<
        crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipSlide))]
    pub(crate) slide_parts: crate::sdk::RepeatedPart<
        crate::parts::slide_part::SlidePart,
    >,
    #[sdk(part_child(relationship_type = RelationshipControl))]
    pub(crate) embedded_control_persistence_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    #[sdk(part_child(relationship_type = RelationshipWebextension))]
    pub(crate) web_extension_parts: crate::sdk::RepeatedPart<
        crate::parts::web_extension_part::WebExtensionPart,
    >,
    #[sdk(part_child(relationship_type = RelationshipComments))]
    pub(crate) comment_parts: crate::sdk::RepeatedPart<
        crate::parts::power_point_comment_part::PowerPointCommentPart,
    >,
}
