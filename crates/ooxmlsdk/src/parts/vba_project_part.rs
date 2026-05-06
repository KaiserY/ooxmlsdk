//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/vbaProject";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str = "application/vnd.ms-office.vbaProject";
pub const TARGET_NAME: &str = "vbaProject";
pub const EXTENSION: &str = ".bin";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct VbaProjectPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl VbaProjectPart {
  crate::sdk_part_child_methods! {
      optional vba_data_part => crate ::parts::vba_data_part::VbaDataPart,
      "http://schemas.microsoft.com/office/2006/relationships/wordVbaData";
  }
}
