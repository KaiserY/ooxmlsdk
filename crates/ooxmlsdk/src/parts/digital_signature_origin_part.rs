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
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub part_content: Vec<u8>,
  pub xml_signature_parts: Vec<crate::parts::xml_signature_part::XmlSignaturePart>,
}
