//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableSingleCells";
pub const PATH_PREFIX: &str = "../tables";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct SingleCellTablePart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element:
    crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::SingleXmlCells,
}
