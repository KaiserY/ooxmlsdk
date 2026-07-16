//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PivotTablePart {
  pub(crate) id: crate::common::PartId,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PivotTableDefinition,
  >,
  pub(crate) pivot_table_cache_definition_part: crate::sdk::RequiredPart<
    crate::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart,
  >,
}
