//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable";
pub const PATH_PREFIX: &str = "../pivotTables";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct PivotTablePart {
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_relationships)]
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  #[sdk(part_rels_path)]
  pub rels_path: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element:
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
    kind = "required"
  ))]
  pub pivot_table_cache_definition_part:
    std::boxed::Box<crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart>,
}
