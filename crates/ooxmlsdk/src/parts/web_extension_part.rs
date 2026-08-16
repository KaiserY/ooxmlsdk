//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct WebExtensionPartSpec {
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension,
  >,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
pub type WebExtensionPart = crate::sdk::PartHandle<WebExtensionPartSpec>;
