//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct TableDefinitionPart {
  pub(crate) key: crate::common::PartKey,
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Table>,
  pub(crate) query_table_parts:
    crate::sdk::RepeatedPart<crate::parts::query_table_part::QueryTablePart>,
}
