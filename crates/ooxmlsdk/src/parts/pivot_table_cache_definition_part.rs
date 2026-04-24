//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition";
pub const PATH_PREFIX: &str = "../pivotCache";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PivotTableCacheDefinitionPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_pivot_table_cache_definition_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords",
    kind = "optional"
  ))]
  pub(crate) pivot_table_cache_records_part:
    crate::sdk::PartChild<crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart>,
}
