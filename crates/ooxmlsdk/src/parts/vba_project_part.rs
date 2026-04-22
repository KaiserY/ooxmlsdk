//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/vbaProject";
pub const PATH_PREFIX: &str = ".";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct VbaProjectPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub part_content: Vec<u8>,
  pub vba_data_part: Option<std::boxed::Box<crate::parts::vba_data_part::VbaDataPart>>,
}
