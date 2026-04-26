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
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/wordVbaData"
  ))]
  pub(crate) vba_data_part: Option<Box<crate::parts::vba_data_part::VbaDataPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl VbaProjectPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] =
    &[crate::sdk::PartChildDescriptor::new(
      "vba_data_part",
      "http://schemas.microsoft.com/office/2006/relationships/wordVbaData",
      "crate::parts::vba_data_part::VbaDataPart",
      crate::sdk::PartChildCardinality::Optional,
    )];
}
