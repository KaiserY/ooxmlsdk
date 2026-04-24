//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/keyMapCustomizations";
pub const PATH_PREFIX: &str = ".";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CustomizationPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_customization_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
    kind = "optional"
  ))]
  pub(crate) word_attached_toolbars_part:
    crate::sdk::PartChild<crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart>,
}
