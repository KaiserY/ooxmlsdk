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
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct EmbeddedControlPersistencePart {
    pub(crate) relationship_id: Option<String>,
    pub(crate) id: crate::common::PartId,
    #[sdk(
        part_child(
            relationship_type = "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary"
        )
    )]
    pub(crate) embedded_control_persistence_binary_data_parts: Vec<
        crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart,
    >,
    pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
    pub(crate) relationship_order: Vec<Box<str>>,
    pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
    pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl EmbeddedControlPersistencePart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
    crate::sdk::PartChildDescriptor::new(
      "embedded_control_persistence_binary_data_parts",
      "http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary",
      "crate::parts::embedded_control_persistence_binary_data_part::EmbeddedControlPersistenceBinaryDataPart",
      crate::sdk::PartChildCardinality::Repeated,
    ),
  ];
}
