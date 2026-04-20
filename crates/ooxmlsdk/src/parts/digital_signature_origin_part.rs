//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin";
pub const PATH_PREFIX: &str = "_xmlsignatures";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct DigitalSignatureOriginPart {
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
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature",
    kind = "repeated"
  ))]
  pub xml_signature_parts: Vec<crate::parts::xml_signature_part::XmlSignaturePart>,
}
