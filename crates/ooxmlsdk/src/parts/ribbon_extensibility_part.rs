//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility";
pub const PATH_PREFIX: &str = "customUI";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct RibbonExtensibilityPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  ))]
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
