//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster";
pub const PATH_PREFIX: &str = "handoutMasters";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct HandoutMasterPart {
    pub(crate) id: crate::common::PartId,
    #[sdk(part_root(accessor = "as_handout_master_part"))]
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::HandoutMaster,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
            kind = "repeated"
        )
    )]
    pub(crate) custom_xml_parts: crate::sdk::PartChild<
        crate::parts::custom_xml_part::CustomXmlPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
            kind = "repeated"
        )
    )]
    pub(crate) chart_parts: crate::sdk::PartChild<crate::parts::chart_part::ChartPart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx",
            kind = "repeated"
        )
    )]
    pub(crate) extended_chart_parts: crate::sdk::PartChild<
        crate::parts::extended_chart_part::ExtendedChartPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
            kind = "repeated"
        )
    )]
    pub(crate) diagram_colors_parts: crate::sdk::PartChild<
        crate::parts::diagram_colors_part::DiagramColorsPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
            kind = "repeated"
        )
    )]
    pub(crate) diagram_data_parts: crate::sdk::PartChild<
        crate::parts::diagram_data_part::DiagramDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
            kind = "repeated"
        )
    )]
    pub(crate) diagram_persist_layout_parts: crate::sdk::PartChild<
        crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
            kind = "repeated"
        )
    )]
    pub(crate) diagram_layout_definition_parts: crate::sdk::PartChild<
        crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
            kind = "repeated"
        )
    )]
    pub(crate) diagram_style_parts: crate::sdk::PartChild<
        crate::parts::diagram_style_part::DiagramStylePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
            kind = "repeated"
        )
    )]
    pub(crate) embedded_object_parts: crate::sdk::PartChild<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
            kind = "repeated"
        )
    )]
    pub(crate) embedded_package_parts: crate::sdk::PartChild<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
            kind = "repeated"
        )
    )]
    pub(crate) image_parts: crate::sdk::PartChild<crate::parts::image_part::ImagePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
            kind = "repeated"
        )
    )]
    pub(crate) vml_drawing_parts: crate::sdk::PartChild<
        crate::parts::vml_drawing_part::VmlDrawingPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
            kind = "repeated"
        )
    )]
    pub(crate) embedded_control_persistence_binary_data_parts: crate::sdk::PartChild<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
            kind = "repeated"
        )
    )]
    pub(crate) model3_d_reference_relationship_parts: crate::sdk::PartChild<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
            kind = "optional"
        )
    )]
    pub(crate) theme_part: crate::sdk::PartChild<crate::parts::theme_part::ThemePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags",
            kind = "repeated"
        )
    )]
    pub(crate) user_defined_tags_parts: crate::sdk::PartChild<
        crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
            kind = "optional"
        )
    )]
    pub(crate) slide_part: crate::sdk::PartChild<crate::parts::slide_part::SlidePart>,
}
