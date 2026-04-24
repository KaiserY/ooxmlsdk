//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility";
pub const PATH_PREFIX: &str = "customUI";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct RibbonAndBackstageCustomizationsPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_ribbon_and_backstage_customizations_part"))]
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    kind = "repeated"
  ))]
  pub(crate) image_parts: crate::sdk::PartChild<crate::parts::image_part::ImagePart>,
}
