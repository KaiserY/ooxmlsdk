//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes";
pub const PATH_PREFIX: &str = "richData";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct RdRichValueTypesPart {
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element:
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichValueTypesInfo,
}
