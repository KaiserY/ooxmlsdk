//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2017/06/relationships/rdArray";
pub const PATH_PREFIX: &str = "richData";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.rdarray+xml";
pub const TARGET_NAME: &str = "rdarray";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct RdArrayPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl RdArrayPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::ArrayData,
    RdArrayPart,
    as_rd_array_part,
    as_rd_array_part_mut
  );
}
