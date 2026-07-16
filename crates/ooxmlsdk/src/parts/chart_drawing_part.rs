//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ChartDrawingPart {
  pub(crate) id: crate::common::PartId,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes,
  >,
  pub(crate) chart_part: crate::sdk::OptionalPart<crate::parts::chart_part::ChartPart>,
  pub(crate) extended_chart_part:
    crate::sdk::OptionalPart<crate::parts::extended_chart_part::ExtendedChartPart>,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
