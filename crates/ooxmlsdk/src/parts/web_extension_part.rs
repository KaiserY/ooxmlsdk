//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2011/relationships/webextension";
pub const PATH_PREFIX: &str = "../webextensions";
pub const CONTENT_TYPE: &str = "application/vnd.ms-office.webextension+xml";
pub const TARGET_NAME: &str = "webextension";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WebExtensionPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl WebExtensionPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_webextensions_webextension_2010_11::WebExtension,
    WebExtensionPart,
    as_web_extension_part,
    as_web_extension_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
  }
}
