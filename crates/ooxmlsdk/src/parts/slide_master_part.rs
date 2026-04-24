//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";
pub const PATH_PREFIX: &str = "slideMasters";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml";
pub const TARGET_NAME: &str = "slideMaster";
pub const EXTENSION: &str = "";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct SlideMasterPart {
    pub(crate) id: crate::common::PartId,
    #[sdk(part_root(accessor = "as_slide_master_part"))]
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::SlideMaster,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"
        )
    )]
    pub(crate) custom_xml_parts: crate::sdk::RepeatedPart<
        crate::parts::custom_xml_part::CustomXmlPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart"
        )
    )]
    pub(crate) chart_parts: crate::sdk::RepeatedPart<
        crate::parts::chart_part::ChartPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx"
        )
    )]
    pub(crate) extended_chart_parts: crate::sdk::RepeatedPart<
        crate::parts::extended_chart_part::ExtendedChartPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors"
        )
    )]
    pub(crate) diagram_colors_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_colors_part::DiagramColorsPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData"
        )
    )]
    pub(crate) diagram_data_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_data_part::DiagramDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing"
        )
    )]
    pub(crate) diagram_persist_layout_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout"
        )
    )]
    pub(crate) diagram_layout_definition_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle"
        )
    )]
    pub(crate) diagram_style_parts: crate::sdk::RepeatedPart<
        crate::parts::diagram_style_part::DiagramStylePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject"
        )
    )]
    pub(crate) embedded_object_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package"
        )
    )]
    pub(crate) embedded_package_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
        )
    )]
    pub(crate) image_parts: crate::sdk::RepeatedPart<
        crate::parts::image_part::ImagePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing"
        )
    )]
    pub(crate) vml_drawing_parts: crate::sdk::RepeatedPart<
        crate::parts::vml_drawing_part::VmlDrawingPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary"
        )
    )]
    pub(crate) embedded_control_persistence_binary_data_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d"
        )
    )]
    pub(crate) model3_d_reference_relationship_parts: crate::sdk::RepeatedPart<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
        )
    )]
    pub(crate) theme_part: crate::sdk::OptionalPart<crate::parts::theme_part::ThemePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide"
        )
    )]
    pub(crate) slide_parts: crate::sdk::RepeatedPart<
        crate::parts::slide_part::SlidePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout"
        )
    )]
    pub(crate) slide_layout_parts: crate::sdk::RepeatedPart<
        crate::parts::slide_layout_part::SlideLayoutPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control"
        )
    )]
    pub(crate) embedded_control_persistence_parts: crate::sdk::RepeatedPart<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags"
        )
    )]
    pub(crate) user_defined_tags_parts: crate::sdk::RepeatedPart<
        crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >,
}
