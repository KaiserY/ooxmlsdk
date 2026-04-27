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
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] =
  &[crate::sdk::PartChildDescriptor::new(
    "vba_data_part",
    "http://schemas.microsoft.com/office/2006/relationships/wordVbaData",
    "crate::parts::vba_data_part::VbaDataPart",
    crate::sdk::PartChildCardinality::Optional,
  )];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct VbaProjectPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl VbaProjectPart {
  pub fn vba_data_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::vba_data_part::VbaDataPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::vba_data_part::VbaDataPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/wordVbaData",
    )
  }
}
