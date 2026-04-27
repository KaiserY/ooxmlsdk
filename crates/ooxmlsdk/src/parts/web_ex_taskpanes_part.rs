//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes";
pub const PATH_PREFIX: &str = "../webextensions";
pub const CONTENT_TYPE: &str = "application/vnd.ms-office.webextensiontaskpanes+xml";
pub const TARGET_NAME: &str = "taskpanes";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] =
  &[crate::sdk::PartChildDescriptor::new(
    "web_extension_parts",
    "http://schemas.microsoft.com/office/2011/relationships/webextension",
    "crate::parts::web_extension_part::WebExtensionPart",
    crate::sdk::PartChildCardinality::Repeated,
  )];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WebExTaskpanesPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl WebExTaskpanesPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes,
    WebExTaskpanesPart,
    as_web_ex_taskpanes_part,
    as_web_ex_taskpanes_part_mut
  );
  #[cfg(feature = "microsoft365")]
  pub fn web_extension_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::web_extension_part::WebExtensionPart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::web_extension_part::WebExtensionPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2011/relationships/webextension",
    )
  }
}
