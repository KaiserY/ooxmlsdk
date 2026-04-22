//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster";
pub const PATH_PREFIX: &str = "notesMasters";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct NotesMasterPart {
    pub r_id: String,
    pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
    pub rels_path: String,
    pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::NotesMaster,
    pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
    pub audio_reference_relationships: Vec<
        crate::common::data_part::AudioReferenceRelationship,
    >,
    pub chart_parts: Vec<crate::parts::chart_part::ChartPart>,
    #[cfg(feature = "microsoft365")]
    pub extended_chart_parts: Vec<crate::parts::extended_chart_part::ExtendedChartPart>,
    pub diagram_colors_parts: Vec<crate::parts::diagram_colors_part::DiagramColorsPart>,
    pub diagram_data_parts: Vec<crate::parts::diagram_data_part::DiagramDataPart>,
    #[cfg(feature = "microsoft365")]
    pub diagram_persist_layout_parts: Vec<
        crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    >,
    pub diagram_layout_definition_parts: Vec<
        crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    >,
    pub diagram_style_parts: Vec<crate::parts::diagram_style_part::DiagramStylePart>,
    pub embedded_object_parts: Vec<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    pub embedded_package_parts: Vec<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    pub image_parts: Vec<crate::parts::image_part::ImagePart>,
    pub video_reference_relationships: Vec<
        crate::common::data_part::VideoReferenceRelationship,
    >,
    pub vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
    pub embedded_control_persistence_binary_data_parts: Vec<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    pub model3_d_reference_relationship_parts: Vec<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    pub theme_part: Option<std::boxed::Box<crate::parts::theme_part::ThemePart>>,
    pub user_defined_tags_parts: Vec<
        crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >,
    pub slide_part: Option<std::boxed::Box<crate::parts::slide_part::SlidePart>>,
}
