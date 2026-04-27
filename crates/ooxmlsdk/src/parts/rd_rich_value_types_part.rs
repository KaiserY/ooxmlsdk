//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueTypes";
pub const PATH_PREFIX: &str = "richData";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.rdrichvaluetypes+xml";
pub const TARGET_NAME: &str = "rdRichValueTypes";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct RdRichValueTypesPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl RdRichValueTypesPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::RichValueTypesInfo,
    RdRichValueTypesPart,
    as_rd_rich_value_types_part,
    as_rd_rich_value_types_part_mut
  );
}
