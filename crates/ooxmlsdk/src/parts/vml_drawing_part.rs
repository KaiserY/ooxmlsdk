//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct VmlDrawingPartSpec {
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  pub(crate) legacy_diagram_text_parts:
    crate::sdk::RepeatedPart<crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart>,
}
pub type VmlDrawingPart = crate::sdk::PartHandle<VmlDrawingPartSpec>;
