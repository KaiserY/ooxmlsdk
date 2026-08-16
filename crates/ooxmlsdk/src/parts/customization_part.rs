//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct CustomizationPartSpec {
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup,
  >,
  pub(crate) word_attached_toolbars_part:
    crate::sdk::OptionalPart<crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart>,
}
pub type CustomizationPart = crate::sdk::PartHandle<CustomizationPartSpec>;
