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
  #[sdk(part_root(accessor = "as_pivot_table_cache_definition_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords"
  ))]
  pub(crate) pivot_table_cache_records_part:
    Option<Box<crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl PivotTableCacheDefinitionPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] =
    &[crate::sdk::PartChildDescriptor::new(
      "pivot_table_cache_records_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheRecords",
      "crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart",
      crate::sdk::PartChildCardinality::Optional,
    )];
}
