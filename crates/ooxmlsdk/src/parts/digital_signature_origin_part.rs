//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin";
pub const PATH_PREFIX: &str = "_xmlsignatures";
pub const CONTENT_TYPE: &str = "application/vnd.openxmlformats-package.digital-signature-origin";
pub const TARGET_NAME: &str = "origin";
pub const EXTENSION: &str = ".sigs";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DigitalSignatureOriginPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl DigitalSignatureOriginPart {
  pub fn xml_signature_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::xml_signature_part::XmlSignaturePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::xml_signature_part::XmlSignaturePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature",
    )
  }
}
