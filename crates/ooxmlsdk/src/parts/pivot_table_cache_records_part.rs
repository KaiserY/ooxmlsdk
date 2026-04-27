//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml";
pub const TARGET_NAME: &str = "pivotCacheRecords";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PivotTableCacheRecordsPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl PivotTableCacheRecordsPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheRecords,
    PivotTableCacheRecordsPart,
    as_pivot_table_cache_records_part,
    as_pivot_table_cache_records_part_mut
  );
}
