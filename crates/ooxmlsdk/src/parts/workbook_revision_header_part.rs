//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionHeaders";
pub const PATH_PREFIX: &str = "revisions";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct WorkbookRevisionHeaderPart {
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_relationships)]
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  #[sdk(part_rels_path)]
  pub rels_path: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element: crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Headers,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/revisionLog",
    kind = "repeated"
  ))]
  pub workbook_revision_log_parts:
    Vec<crate::parts::workbook_revision_log_part::WorkbookRevisionLogPart>,
}
