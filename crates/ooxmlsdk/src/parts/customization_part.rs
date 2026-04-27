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
}
impl CustomizationPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_word_2006_wordml::TemplateCommandGroup,
    CustomizationPart,
    as_customization_part,
    as_customization_part_mut
  );
  pub fn word_attached_toolbars_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::word_attached_toolbars_part::WordAttachedToolbarsPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2006/relationships/attachedToolbars",
    )
  }
}
