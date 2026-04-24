//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes";
pub const PATH_PREFIX: &str = "../webextensions";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WebExTaskpanesPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_web_ex_taskpanes_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextension"
  ))]
  pub(crate) web_extension_parts:
    crate::sdk::RepeatedPart<crate::parts::web_extension_part::WebExtensionPart>,
}
