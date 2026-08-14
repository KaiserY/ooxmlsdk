//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PivotTableCacheDefinitionPart {
  pub(crate) key: crate::common::PartKey,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
  >,
  pub(crate) pivot_table_cache_records_part: crate::sdk::OptionalPart<
    crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart,
  >,
}
