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
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature"
  ))]
  pub(crate) xml_signature_parts: Vec<crate::parts::xml_signature_part::XmlSignaturePart>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl DigitalSignatureOriginPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] =
    &[crate::sdk::PartChildDescriptor::new(
      "xml_signature_parts",
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature",
      "crate::parts::xml_signature_part::XmlSignaturePart",
      crate::sdk::PartChildCardinality::Repeated,
    )];
}
