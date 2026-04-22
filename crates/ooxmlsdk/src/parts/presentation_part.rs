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
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element:
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
  pub custom_xml_parts: Vec<crate::parts::custom_xml_part::CustomXmlPart>,
  pub font_parts: Vec<crate::parts::font_part::FontPart>,
  pub presentation_properties_part:
    Option<std::boxed::Box<crate::parts::presentation_properties_part::PresentationPropertiesPart>>,
  pub table_styles_part: Option<std::boxed::Box<crate::parts::table_styles_part::TableStylesPart>>,
  pub theme_part: Option<std::boxed::Box<crate::parts::theme_part::ThemePart>>,
  pub view_properties_part:
    Option<std::boxed::Box<crate::parts::view_properties_part::ViewPropertiesPart>>,
  pub notes_master_part: Option<std::boxed::Box<crate::parts::notes_master_part::NotesMasterPart>>,
  pub slide_parts: Vec<crate::parts::slide_part::SlidePart>,
  pub slide_master_parts: Vec<crate::parts::slide_master_part::SlideMasterPart>,
  pub user_defined_tags_part:
    Option<std::boxed::Box<crate::parts::user_defined_tags_part::UserDefinedTagsPart>>,
  pub comment_authors_part:
    Option<std::boxed::Box<crate::parts::comment_authors_part::CommentAuthorsPart>>,
  pub handout_master_part:
    Option<std::boxed::Box<crate::parts::handout_master_part::HandoutMasterPart>>,
  pub legacy_diagram_text_info_part:
    Option<std::boxed::Box<crate::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart>>,
  pub vba_project_part: Option<std::boxed::Box<crate::parts::vba_project_part::VbaProjectPart>>,
  #[cfg(feature = "microsoft365")]
  pub comment_parts: Vec<crate::parts::power_point_comment_part::PowerPointCommentPart>,
  #[cfg(feature = "microsoft365")]
  pub authors_part:
    Option<std::boxed::Box<crate::parts::power_point_authors_part::PowerPointAuthorsPart>>,
}
