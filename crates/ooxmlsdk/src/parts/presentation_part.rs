//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub const PATH_PREFIX: &str = "ppt";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PresentationPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_presentation_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml"
  ))]
  pub(crate) custom_xml_parts:
    crate::sdk::RepeatedPart<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font"
  ))]
  pub(crate) font_parts: crate::sdk::RepeatedPart<crate::parts::font_part::FontPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps"
  ))]
  pub(crate) presentation_properties_part: crate::sdk::OptionalPart<
    crate::parts::presentation_properties_part::PresentationPropertiesPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles"
  ))]
  pub(crate) table_styles_part:
    crate::sdk::OptionalPart<crate::parts::table_styles_part::TableStylesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
  ))]
  pub(crate) theme_part: crate::sdk::OptionalPart<crate::parts::theme_part::ThemePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps"
  ))]
  pub(crate) view_properties_part:
    crate::sdk::OptionalPart<crate::parts::view_properties_part::ViewPropertiesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster"
  ))]
  pub(crate) notes_master_part:
    crate::sdk::OptionalPart<crate::parts::notes_master_part::NotesMasterPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide"
  ))]
  pub(crate) slide_parts: crate::sdk::RepeatedPart<crate::parts::slide_part::SlidePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster"
  ))]
  pub(crate) slide_master_parts:
    crate::sdk::RepeatedPart<crate::parts::slide_master_part::SlideMasterPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags"
  ))]
  pub(crate) user_defined_tags_part:
    crate::sdk::OptionalPart<crate::parts::user_defined_tags_part::UserDefinedTagsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors"
  ))]
  pub(crate) comment_authors_part:
    crate::sdk::OptionalPart<crate::parts::comment_authors_part::CommentAuthorsPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster"
  ))]
  pub(crate) handout_master_part:
    crate::sdk::OptionalPart<crate::parts::handout_master_part::HandoutMasterPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo"
  ))]
  pub(crate) legacy_diagram_text_info_part: crate::sdk::OptionalPart<
    crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject"
  ))]
  pub(crate) vba_project_part:
    crate::sdk::OptionalPart<crate::parts::vba_project_part::VbaProjectPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/comments"
  ))]
  pub(crate) comment_parts:
    crate::sdk::RepeatedPart<crate::parts::power_point_comment_part::PowerPointCommentPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/authors"
  ))]
  pub(crate) authors_part:
    crate::sdk::OptionalPart<crate::parts::power_point_authors_part::PowerPointAuthorsPart>,
}
