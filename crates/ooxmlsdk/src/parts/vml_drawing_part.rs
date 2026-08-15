//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, Hash, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct VmlDrawingPart {
  pub(crate) key: crate::common::PartKey,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  pub(crate) legacy_diagram_text_parts:
    crate::sdk::RepeatedPart<crate::parts::legacy_diagram_text_part::LegacyDiagramTextPart>,
}
