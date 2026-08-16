//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct PivotTablePartSpec {
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition,
  >,
  pub(crate) pivot_table_cache_definition_part: crate::sdk::RequiredPart<
    crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
  >,
}
pub type PivotTablePart = crate::sdk::PartHandle<PivotTablePartSpec>;
