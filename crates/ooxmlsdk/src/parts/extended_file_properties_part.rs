//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties";
pub const PATH_PREFIX: &str = "docProps";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct ExtendedFilePropertiesPart {
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element:
    crate::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties,
}
