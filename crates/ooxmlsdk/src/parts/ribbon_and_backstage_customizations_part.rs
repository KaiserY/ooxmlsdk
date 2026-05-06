//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility";
pub const PATH_PREFIX: &str = "customUI";
pub const CONTENT_TYPE: &str = "application/xml";
pub const TARGET_NAME: &str = "customUI";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct RibbonAndBackstageCustomizationsPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl RibbonAndBackstageCustomizationsPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi,
    RibbonAndBackstageCustomizationsPart,
    as_ribbon_and_backstage_customizations_part,
    as_ribbon_and_backstage_customizations_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
  }
}
