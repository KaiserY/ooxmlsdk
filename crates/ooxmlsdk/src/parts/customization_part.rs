//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str = "application/vnd.ms-word.keyMapCustomizations+xml";
pub const TARGET_NAME: &str = "customizations";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CustomizationPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_customization_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars"
  ))]
  pub(crate) word_attached_toolbars_part:
    Option<Box<crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl CustomizationPart {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] =
    &[crate::sdk::PartChildDescriptor::new(
      "word_attached_toolbars_part",
      "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
      "crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart",
      crate::sdk::PartChildCardinality::Optional,
    )];
}
