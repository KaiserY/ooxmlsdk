//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable";
pub const PATH_PREFIX: &str = "../pivotTables";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PivotTablePart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_pivot_table_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
    kind = "required"
  ))]
  pub(crate) pivot_table_cache_definition_part: crate::sdk::PartChild<
    crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
  >,
}
