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
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    kind = "repeated"
  ))]
  pub(crate) custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font",
    kind = "repeated"
  ))]
  pub(crate) font_parts: Vec<crate::parts::font_part::FontPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps",
    kind = "optional"
  ))]
  pub(crate) presentation_properties_part:
    Option<Box<crate::parts::presentation_properties_part::PresentationPropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles",
    kind = "optional"
  ))]
  pub(crate) table_styles_part: Option<Box<crate::parts::table_styles_part::TableStylesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    kind = "optional"
  ))]
  pub(crate) theme_part: Option<Box<crate::parts::theme_part::ThemePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps",
    kind = "optional"
  ))]
  pub(crate) view_properties_part:
    Option<Box<crate::parts::view_properties_part::ViewPropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster",
    kind = "optional"
  ))]
  pub(crate) notes_master_part: Option<Box<crate::parts::notes_master_part::NotesMasterPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
    kind = "repeated"
  ))]
  pub(crate) slide_parts: Vec<crate::parts::slide_part::SlidePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster",
    kind = "required_repeated"
  ))]
  pub(crate) slide_master_parts: Vec<crate::parts::slide_master_part::SlideMasterPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags",
    kind = "optional"
  ))]
  pub(crate) user_defined_tags_part:
    Option<Box<crate::parts::user_defined_tags_part::UserDefinedTagsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors",
    kind = "optional"
  ))]
  pub(crate) comment_authors_part:
    Option<Box<crate::parts::comment_authors_part::CommentAuthorsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster",
    kind = "optional"
  ))]
  pub(crate) handout_master_part: Option<Box<crate::parts::handout_master_part::HandoutMasterPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo",
    kind = "optional"
  ))]
  pub(crate) legacy_diagram_text_info_part:
    Option<Box<crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
    kind = "optional"
  ))]
  pub(crate) vba_project_part: Option<Box<crate::parts::vba_project_part::VbaProjectPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/comments",
    kind = "repeated"
  ))]
  pub(crate) comment_parts: Vec<crate::parts::power_point_comment_part::PowerPointCommentPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/authors",
    kind = "optional"
  ))]
  pub(crate) authors_part:
    Option<Box<crate::parts::power_point_authors_part::PowerPointAuthorsPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) modeled_relationships: Vec<crate::common::RelationshipInfo>,
}
