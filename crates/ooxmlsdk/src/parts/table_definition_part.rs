//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table";
pub const PATH_PREFIX: &str = "../tables";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml";
pub const TARGET_NAME: &str = "table";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct TableDefinitionPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl TableDefinitionPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table,
    TableDefinitionPart,
    as_table_definition_part,
    as_table_definition_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated query_table_parts => crate ::parts::query_table_part::QueryTablePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/queryTable";
  }
}
