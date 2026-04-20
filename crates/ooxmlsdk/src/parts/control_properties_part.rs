//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp";
pub const PATH_PREFIX: &str = "../ctrlProps";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct ControlPropertiesPart {
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element:
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FormControlProperties,
}
