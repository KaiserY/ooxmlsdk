//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct ChartDrawingPartSpec {
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes,
  >,
  pub(crate) chart_part: crate::sdk::OptionalPart<crate::parts::chart_part::ChartPart>,
  pub(crate) extended_chart_part:
    crate::sdk::OptionalPart<crate::parts::extended_chart_part::ExtendedChartPart>,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
pub type ChartDrawingPart = crate::sdk::PartHandle<ChartDrawingPartSpec>;
