//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, Hash, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DigitalSignatureOriginPart {
  pub(crate) key: crate::common::PartKey,
  pub(crate) xml_signature_parts:
    crate::sdk::RepeatedPart<crate::parts::xml_signature_part::XmlSignaturePart>,
}
