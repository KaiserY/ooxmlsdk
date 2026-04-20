//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
pub const PATH_PREFIX: &str = "ppt";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct PresentationPart {
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_relationships)]
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  #[sdk(part_rels_path)]
  pub rels_path: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element:
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml",
    kind = "repeated"
  ))]
  pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font",
    kind = "repeated"
  ))]
  pub font_parts: Vec<crate::parts::font_part::FontPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps",
    kind = "optional"
  ))]
  pub presentation_properties_part:
    Option<std::boxed::Box<crate::parts::presentation_properties_part::PresentationPropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles",
    kind = "optional"
  ))]
  pub table_styles_part: Option<std::boxed::Box<crate::parts::table_styles_part::TableStylesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme",
    kind = "optional"
  ))]
  pub theme_part: Option<std::boxed::Box<crate::parts::theme_part::ThemePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps",
    kind = "optional"
  ))]
  pub view_properties_part:
    Option<std::boxed::Box<crate::parts::view_properties_part::ViewPropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster",
    kind = "optional"
  ))]
  pub notes_master_part: Option<std::boxed::Box<crate::parts::notes_master_part::NotesMasterPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
    kind = "repeated"
  ))]
  pub slide_parts: Vec<crate::parts::slide_part::SlidePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster",
    kind = "repeated"
  ))]
  pub slide_master_parts: Vec<crate::parts::slide_master_part::SlideMasterPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags",
    kind = "optional"
  ))]
  pub user_defined_tags_part:
    Option<std::boxed::Box<crate::parts::user_defined_tags_part::UserDefinedTagsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors",
    kind = "optional"
  ))]
  pub comment_authors_part:
    Option<std::boxed::Box<crate::parts::comment_authors_part::CommentAuthorsPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster",
    kind = "optional"
  ))]
  pub handout_master_part:
    Option<std::boxed::Box<crate::parts::handout_master_part::HandoutMasterPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo",
    kind = "optional"
  ))]
  pub legacy_diagram_text_info_part:
    Option<std::boxed::Box<crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/vbaProject",
    kind = "optional"
  ))]
  pub vba_project_part: Option<std::boxed::Box<crate::parts::vba_project_part::VbaProjectPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/comments",
    kind = "repeated"
  ))]
  pub comment_parts: Vec<crate::parts::power_point_comment_part::PowerPointCommentPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2018/10/relationships/authors",
    kind = "optional"
  ))]
  pub authors_part:
    Option<std::boxed::Box<crate::parts::power_point_authors_part::PowerPointAuthorsPart>>,
}
