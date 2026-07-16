//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PresentationPart {
  pub(crate) id: crate::common::PartId,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
  >,
  pub(crate) custom_xml_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_xml_part::CustomXmlPart>,
  pub(crate) font_parts: crate::sdk::RepeatedPart<crate::parts::font_part::FontPart>,
  pub(crate) presentation_properties_part: crate::sdk::OptionalPart<
    crate::parts::presentation_properties_part::PresentationPropertiesPart,
  >,
  pub(crate) table_styles_part:
    crate::sdk::OptionalPart<crate::parts::table_styles_part::TableStylesPart>,
  pub(crate) theme_part: crate::sdk::OptionalPart<crate::parts::theme_part::ThemePart>,
  pub(crate) view_properties_part:
    crate::sdk::OptionalPart<crate::parts::view_properties_part::ViewPropertiesPart>,
  pub(crate) notes_master_part:
    crate::sdk::OptionalPart<crate::parts::notes_master_part::NotesMasterPart>,
  pub(crate) slide_parts: crate::sdk::RepeatedPart<crate::parts::slide_part::SlidePart>,
  #[sdk(part_child(kind = "required_repeated"))]
  pub(crate) slide_master_parts:
    crate::sdk::RepeatedPart<crate::parts::slide_master_part::SlideMasterPart>,
  pub(crate) user_defined_tags_part:
    crate::sdk::OptionalPart<crate::parts::user_defined_tags_part::UserDefinedTagsPart>,
  pub(crate) comment_authors_part:
    crate::sdk::OptionalPart<crate::parts::comment_authors_part::CommentAuthorsPart>,
  pub(crate) handout_master_part:
    crate::sdk::OptionalPart<crate::parts::handout_master_part::HandoutMasterPart>,
  pub(crate) legacy_diagram_text_info_part: crate::sdk::OptionalPart<
    crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
  >,
  pub(crate) vba_project_part:
    crate::sdk::OptionalPart<crate::parts::vba_project_part::VbaProjectPart>,
  pub(crate) comment_parts:
    crate::sdk::RepeatedPart<crate::parts::power_point_comment_part::PowerPointCommentPart>,
  pub(crate) authors_part:
    crate::sdk::OptionalPart<crate::parts::power_point_authors_part::PowerPointAuthorsPart>,
}
