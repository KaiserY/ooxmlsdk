//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str = "";
pub const TARGET_NAME: &str = "ActiveXControl";
pub const EXTENSION: &str = ".bin";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct EmbeddedControlPersistenceBinaryDataPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
