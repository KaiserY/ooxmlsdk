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
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_relationships)]
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  #[sdk(part_rels_path)]
  pub rels_path: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_content(kind = "binary"))]
  pub part_content: Vec<u8>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/wordVbaData",
    kind = "optional"
  ))]
  pub vba_data_part: Option<std::boxed::Box<crate::parts::vba_data_part::VbaDataPart>>,
}
