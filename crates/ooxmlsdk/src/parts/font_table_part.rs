//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable";
pub const PATH_PREFIX: &str = ".";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct FontTablePart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_font_table_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font"
  ))]
  pub(crate) font_parts: crate::sdk::RepeatedPart<crate::parts::font_part::FontPart>,
}
