//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct ThemePartSpec {
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Theme>,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
pub type ThemePart = crate::sdk::PartHandle<ThemePartSpec>;
