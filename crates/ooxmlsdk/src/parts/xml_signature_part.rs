//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature";
pub const PATH_PREFIX: &str = "_xmlsignatures";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct XmlSignaturePart {
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_content(kind = "text"))]
  pub part_content: String,
}
