//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
pub const PATH_PREFIX: &str = "slides";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct SlidePart {
    #[sdk(part_rid)]
    pub r_id: String,
    #[sdk(part_relationships)]
    pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
    #[sdk(part_rels_path)]
    pub rels_path: String,
    #[sdk(part_inner_path)]
    pub inner_path: String,
    #[sdk(part_root)]
    pub root_element: crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Slide,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
            kind = "repeated"
        )
    )]
    pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/audio",
            kind = "repeated"
        )
    )]
    pub audio_reference_relationships: Vec<crate::parts::media_data_part::MediaDataPart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
            kind = "repeated"
        )
    )]
    pub chart_parts: Vec<crate::parts::chart_part::ChartPart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2014/relationships/chartEx",
            kind = "repeated"
        )
    )]
    pub extended_chart_parts: Vec<crate::parts::extended_chart_part::ExtendedChartPart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramColors",
            kind = "repeated"
        )
    )]
    pub diagram_colors_parts: Vec<crate::parts::diagram_colors_part::DiagramColorsPart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramData",
            kind = "repeated"
        )
    )]
    pub diagram_data_parts: Vec<crate::parts::diagram_data_part::DiagramDataPart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2007/relationships/diagramDrawing",
            kind = "repeated"
        )
    )]
    pub diagram_persist_layout_parts: Vec<
        crate::parts::diagram_persist_layout_part::DiagramPersistLayoutPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramLayout",
            kind = "repeated"
        )
    )]
    pub diagram_layout_definition_parts: Vec<
        crate::parts::diagram_layout_definition_part::DiagramLayoutDefinitionPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/diagramQuickStyle",
            kind = "repeated"
        )
    )]
    pub diagram_style_parts: Vec<crate::parts::diagram_style_part::DiagramStylePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
            kind = "repeated"
        )
    )]
    pub embedded_object_parts: Vec<
        crate::parts::embedded_object_part::EmbeddedObjectPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
            kind = "repeated"
        )
    )]
    pub embedded_package_parts: Vec<
        crate::parts::embedded_package_part::EmbeddedPackagePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
            kind = "repeated"
        )
    )]
    pub image_parts: Vec<crate::parts::image_part::ImagePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
            kind = "repeated"
        )
    )]
    pub vml_drawing_parts: Vec<crate::parts::vml_drawing_part::VmlDrawingPart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
            kind = "repeated"
        )
    )]
    pub embedded_control_persistence_binary_data_parts: Vec<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2017/06/relationships/model3d",
            kind = "repeated"
        )
    )]
    pub model3_d_reference_relationship_parts: Vec<
        crate::parts::model3_d_reference_relationship_part::Model3DReferenceRelationshipPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
            kind = "optional"
        )
    )]
    pub slide_comments_part: Option<
        std::boxed::Box<crate::parts::slide_comments_part::SlideCommentsPart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide",
            kind = "optional"
        )
    )]
    pub notes_slide_part: Option<
        std::boxed::Box<crate::parts::notes_slide_part::NotesSlidePart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride",
            kind = "optional"
        )
    )]
    pub theme_override_part: Option<
        std::boxed::Box<crate::parts::theme_override_part::ThemeOverridePart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout",
            kind = "optional"
        )
    )]
    pub slide_layout_part: Option<
        std::boxed::Box<crate::parts::slide_layout_part::SlideLayoutPart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideUpdateInfo",
            kind = "optional"
        )
    )]
    pub slide_sync_data_part: Option<
        std::boxed::Box<crate::parts::slide_sync_data_part::SlideSyncDataPart>,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags",
            kind = "repeated"
        )
    )]
    pub user_defined_tags_parts: Vec<
        crate::parts::user_defined_tags_part::UserDefinedTagsPart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
            kind = "repeated"
        )
    )]
    pub slide_parts: Vec<crate::parts::slide_part::SlidePart>,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control",
            kind = "repeated"
        )
    )]
    pub embedded_control_persistence_parts: Vec<
        crate::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart,
    >,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2007/relationships/media",
            kind = "repeated"
        )
    )]
    pub media_reference_relationships: Vec<crate::parts::media_data_part::MediaDataPart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextension",
            kind = "repeated"
        )
    )]
    pub web_extension_parts: Vec<crate::parts::web_extension_part::WebExtensionPart>,
    #[cfg(feature = "microsoft365")]
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/comments",
            kind = "repeated"
        )
    )]
    pub comment_parts: Vec<
        crate::parts::power_point_comment_part::PowerPointCommentPart,
    >,
}
