//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct VmlDrawingPart {
  pub(crate) id: crate::common::PartId,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  pub(crate) legacy_diagram_text_parts:
    crate::sdk::RepeatedPart<crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart>,
}
