//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/table";
pub const PATH_PREFIX: &str = "../tables";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct TableDefinitionPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table,
  pub query_table_parts: Vec<crate::parts::query_table_part::QueryTablePart>,
}
