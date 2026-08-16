//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct WorkbookRevisionHeaderPartSpec {
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers,
  >,
  pub(crate) workbook_revision_log_parts:
    crate::sdk::RepeatedPart<crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart>,
}
pub type WorkbookRevisionHeaderPart = crate::sdk::PartHandle<WorkbookRevisionHeaderPartSpec>;
