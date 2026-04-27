//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature";
pub const PATH_PREFIX: &str = "_xmlsignatures";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml";
pub const TARGET_NAME: &str = "sig";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct XmlSignaturePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
