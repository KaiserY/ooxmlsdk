//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin";
pub const PATH_PREFIX: &str = "_xmlsignatures";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DigitalSignatureOriginPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature"
  ))]
  pub(crate) xml_signature_parts:
    crate::sdk::RepeatedPart<crate::parts::xml_signature_part::XmlSignaturePart>,
}
