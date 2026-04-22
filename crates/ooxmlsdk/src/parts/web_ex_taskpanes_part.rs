//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes";
pub const PATH_PREFIX: &str = "../webextensions";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct WebExTaskpanesPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element:
    crate::schemas::schemas_microsoft_com_office_webextensions_taskpanes_2010_11::Taskpanes,
  #[cfg(feature = "microsoft365")]
  pub web_extension_parts: Vec<crate::parts::web_extension_part::WebExtensionPart>,
}
