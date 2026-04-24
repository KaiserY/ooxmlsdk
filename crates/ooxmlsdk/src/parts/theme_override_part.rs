//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride";
pub const PATH_PREFIX: &str = "theme";
pub const CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.themeOverride+xml";
pub const TARGET_NAME: &str = "themeoverride";
pub const EXTENSION: &str = "";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ThemeOverridePart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_theme_override_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ThemeOverride,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  ))]
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
