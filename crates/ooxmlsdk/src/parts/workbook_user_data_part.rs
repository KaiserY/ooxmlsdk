//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/usernames";
pub const PATH_PREFIX: &str = "revisions";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct WorkbookUserDataPart {
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Users,
}
