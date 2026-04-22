//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition";
pub const PATH_PREFIX: &str = "../pivotCache";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct PivotTableCacheDefinitionPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element:
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
  pub pivot_table_cache_records_part: Option<
    std::boxed::Box<crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart>,
  >,
}
