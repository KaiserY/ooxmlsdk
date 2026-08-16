//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct TableDefinitionPartSpec {
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table>,
  pub(crate) query_table_parts:
    crate::sdk::RepeatedPart<crate::parts::query_table_part::QueryTablePart>,
}
pub type TableDefinitionPart = crate::sdk::PartHandle<TableDefinitionPartSpec>;
