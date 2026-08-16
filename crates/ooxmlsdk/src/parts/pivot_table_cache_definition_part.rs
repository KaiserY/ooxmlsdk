//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct PivotTableCacheDefinitionPartSpec {
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotCacheDefinition,
  >,
  pub(crate) pivot_table_cache_records_part: crate::sdk::OptionalPart<
    crate::parts::pivot_table_cache_records_part::PivotTableCacheRecordsPart,
  >,
}
pub type PivotTableCacheDefinitionPart = crate::sdk::PartHandle<PivotTableCacheDefinitionPartSpec>;
