//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, Hash, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorkbookRevisionHeaderPart {
  pub(crate) key: crate::common::PartKey,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers,
  >,
  pub(crate) workbook_revision_log_parts:
    crate::sdk::RepeatedPart<crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart>,
}
