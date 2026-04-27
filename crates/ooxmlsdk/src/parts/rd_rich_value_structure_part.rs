//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2017/06/relationships/rdRichValueStructure";
pub const PATH_PREFIX: &str = "richData";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.rdrichvaluestructure+xml";
pub const TARGET_NAME: &str = "rdrichvaluestructure";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct RdRichValueStructurePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl RdRichValueStructurePart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata::RichValueStructures,
    RdRichValueStructurePart,
    as_rd_rich_value_structure_part,
    as_rd_rich_value_structure_part_mut
  );
}
