//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition";
pub const PATH_PREFIX: &str = "../pivotCache";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml";
pub const TARGET_NAME: &str = "pivotCacheDefinition";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PivotTableCacheDefinitionPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl PivotTableCacheDefinitionPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
    PivotTableCacheDefinitionPart,
    as_pivot_table_cache_definition_part,
    as_pivot_table_cache_definition_part_mut
  );
  pub fn pivot_table_cache_records_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords",
    )
  }
}
