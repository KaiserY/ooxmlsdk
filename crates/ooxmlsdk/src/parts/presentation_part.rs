//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub const PATH_PREFIX: &str = "ppt";
pub const CONTENT_TYPE: &str = "";
pub const TARGET_NAME: &str = "presentation";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PresentationPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_presentation_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
  >,
  #[sdk(part_child(relationship_type = RelationshipCustomXml))]
  pub(crate) custom_xml_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(relationship_type = RelationshipFont))]
  pub(crate) font_parts: crate::sdk::RepeatedPart<crate::parts::font_part::FontPart>,
  #[sdk(part_child(relationship_type = RelationshipPresProps))]
  pub(crate) presentation_properties_part: crate::sdk::OptionalPart<
    crate::parts::presentation_properties_part::PresentationPropertiesPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipTableStyles))]
  pub(crate) table_styles_part:
    crate::sdk::OptionalPart<crate::parts::table_styles_part::TableStylesPart>,
  #[sdk(part_child(relationship_type = RelationshipTheme))]
  pub(crate) theme_part: crate::sdk::OptionalPart<crate::parts::theme_part::ThemePart>,
  #[sdk(part_child(relationship_type = RelationshipViewProps))]
  pub(crate) view_properties_part:
    crate::sdk::OptionalPart<crate::parts::view_properties_part::ViewPropertiesPart>,
  #[sdk(part_child(relationship_type = RelationshipNotesMaster))]
  pub(crate) notes_master_part:
    crate::sdk::OptionalPart<crate::parts::notes_master_part::NotesMasterPart>,
  #[sdk(part_child(relationship_type = RelationshipSlide))]
  pub(crate) slide_parts: crate::sdk::RepeatedPart<crate::parts::slide_part::SlidePart>,
  #[sdk(
        part_child(
            relationship_type = RelationshipSlideMaster,
            kind = "required_repeated"
        )
    )]
  pub(crate) slide_master_parts:
    crate::sdk::RepeatedPart<crate::parts::slide_master_part::SlideMasterPart>,
  #[sdk(part_child(relationship_type = RelationshipTags))]
  pub(crate) user_defined_tags_part:
    crate::sdk::OptionalPart<crate::parts::user_defined_tags_part::UserDefinedTagsPart>,
  #[sdk(part_child(relationship_type = RelationshipCommentAuthors))]
  pub(crate) comment_authors_part:
    crate::sdk::OptionalPart<crate::parts::comment_authors_part::CommentAuthorsPart>,
  #[sdk(part_child(relationship_type = RelationshipHandoutMaster))]
  pub(crate) handout_master_part:
    crate::sdk::OptionalPart<crate::parts::handout_master_part::HandoutMasterPart>,
  #[sdk(part_child(relationship_type = RelationshipLegacyDocTextInfo))]
  pub(crate) legacy_diagram_text_info_part: crate::sdk::OptionalPart<
    crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
  >,
  #[sdk(part_child(relationship_type = RelationshipVbaProject))]
  pub(crate) vba_project_part:
    crate::sdk::OptionalPart<crate::parts::vba_project_part::VbaProjectPart>,
  #[sdk(part_child(relationship_type = RelationshipComments))]
  pub(crate) comment_parts:
    crate::sdk::RepeatedPart<crate::parts::power_point_comment_part::PowerPointCommentPart>,
  #[sdk(part_child(relationship_type = RelationshipAuthors))]
  pub(crate) authors_part:
    crate::sdk::OptionalPart<crate::parts::power_point_authors_part::PowerPointAuthorsPart>,
}
