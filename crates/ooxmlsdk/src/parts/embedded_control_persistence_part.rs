//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/control";
pub const PATH_PREFIX: &str = "embeddings";
pub const CONTENT_TYPE: &str = "";
pub const TARGET_NAME: &str = "control";
pub const EXTENSION: &str = ".bin";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[
  crate::sdk::PartChildDescriptor::new(
    "embedded_control_persistence_binary_data_parts",
    "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
    "crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct EmbeddedControlPersistencePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl EmbeddedControlPersistencePart {
  pub fn embedded_control_persistence_binary_data_parts<'a, P: crate::sdk::SdkPackage>(
        &'a self,
        package: &'a P,
    ) -> impl Iterator<
        Item = crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
  > + 'a{
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
            P,
            crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
        >(
            self,
            package,
            "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
        )
  }
}
