//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders";
pub const PATH_PREFIX: &str = "revisions";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorkbookRevisionHeaderPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_workbook_revision_header_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog"
  ))]
  pub(crate) workbook_revision_log_parts:
    crate::sdk::RepeatedPart<crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart>,
}
