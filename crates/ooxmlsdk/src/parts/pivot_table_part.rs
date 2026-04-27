//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotTable";
pub const PATH_PREFIX: &str = "../pivotTables";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml";
pub const TARGET_NAME: &str = "pivotTable";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] =
  &[crate::sdk::PartChildDescriptor::new(
    "pivot_table_cache_definition_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
    "crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart",
    crate::sdk::PartChildCardinality::Required,
  )];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PivotTablePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl PivotTablePart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition,
    PivotTablePart,
    as_pivot_table_part,
    as_pivot_table_part_mut
  );
  pub fn pivot_table_cache_definition_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition",
    )
  }
}
