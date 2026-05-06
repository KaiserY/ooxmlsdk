//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml";
pub const TARGET_NAME: &str = "fontTable";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct FontTablePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl FontTablePart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts,
    FontTablePart,
    as_font_table_part,
    as_font_table_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated font_parts => crate ::parts::font_part::FontPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font";
  }
}
