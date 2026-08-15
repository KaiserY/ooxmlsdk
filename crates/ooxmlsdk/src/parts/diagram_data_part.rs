//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, Hash, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DiagramDataPart {
  pub(crate) key: crate::common::PartKey,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_diagram::DataModelRoot,
  >,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  pub(crate) slide_parts: crate::sdk::RepeatedPart<crate::parts::slide_part::SlidePart>,
  pub(crate) worksheet_parts: crate::sdk::RepeatedPart<crate::parts::worksheet_part::WorksheetPart>,
}
