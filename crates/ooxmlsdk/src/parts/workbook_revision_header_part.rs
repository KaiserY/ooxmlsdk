//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders";
pub const PATH_PREFIX: &str = "revisions";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml";
pub const TARGET_NAME: &str = "revisionHeaders";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WorkbookRevisionHeaderPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl WorkbookRevisionHeaderPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers,
    WorkbookRevisionHeaderPart,
    as_workbook_revision_header_part,
    as_workbook_revision_header_part_mut
  );
  pub fn workbook_revision_log_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart> + 'a
  {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog",
    )
  }
}
