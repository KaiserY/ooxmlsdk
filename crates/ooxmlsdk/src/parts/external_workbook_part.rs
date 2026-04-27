//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/externalLink";
pub const PATH_PREFIX: &str = "externalReferences";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml";
pub const TARGET_NAME: &str = "externalReference";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ExternalWorkbookPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl ExternalWorkbookPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::ExternalLink,
    ExternalWorkbookPart,
    as_external_workbook_part,
    as_external_workbook_part_mut
  );
}
