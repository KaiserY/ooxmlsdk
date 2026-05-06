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
}
impl PresentationPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Presentation,
    PresentationPart,
    as_presentation_part,
    as_presentation_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated custom_xml_parts => crate ::parts::custom_xml_part::CustomXmlPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml";
      repeated font_parts => crate ::parts::font_part::FontPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font";
      optional presentation_properties_part => crate
      ::parts::presentation_properties_part::PresentationPropertiesPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps";
      optional table_styles_part => crate ::parts::table_styles_part::TableStylesPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles";
      optional theme_part => crate ::parts::theme_part::ThemePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
      optional view_properties_part => crate
      ::parts::view_properties_part::ViewPropertiesPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps";
      optional notes_master_part => crate ::parts::notes_master_part::NotesMasterPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster";
      repeated slide_parts => crate ::parts::slide_part::SlidePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide";
      repeated slide_master_parts => crate ::parts::slide_master_part::SlideMasterPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster";
      optional user_defined_tags_part => crate
      ::parts::user_defined_tags_part::UserDefinedTagsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tags";
      optional comment_authors_part => crate
      ::parts::comment_authors_part::CommentAuthorsPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/commentAuthors";
      optional handout_master_part => crate
      ::parts::handout_master_part::HandoutMasterPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/handoutMaster";
      optional legacy_diagram_text_info_part => crate
      ::parts::legacy_diagram_text_info_part::LegacyDiagramTextInfoPart,
      "http://schemas.microsoft.com/office/2006/relationships/legacyDocTextInfo";
      optional vba_project_part => crate ::parts::vba_project_part::VbaProjectPart,
      "http://schemas.microsoft.com/office/2006/relationships/vbaProject"; repeated
      comment_parts => crate ::parts::power_point_comment_part::PowerPointCommentPart,
      "http://schemas.microsoft.com/office/2018/10/relationships/comments"; optional
      authors_part => crate ::parts::power_point_authors_part::PowerPointAuthorsPart,
      "http://schemas.microsoft.com/office/2018/10/relationships/authors";
  }
}
