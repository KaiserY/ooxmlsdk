//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ChartPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace,
  >,
  pub(crate) chart_drawing_part:
    crate::sdk::OptionalPart<crate::parts::chart_drawing_part::ChartDrawingPart>,
  pub(crate) embedded_package_part:
    crate::sdk::OptionalPart<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  pub(crate) theme_override_part:
    crate::sdk::OptionalPart<crate::parts::theme_override_part::ThemeOverridePart>,
  pub(crate) chart_style_parts:
    crate::sdk::RepeatedPart<crate::parts::chart_style_part::ChartStylePart>,
  pub(crate) chart_color_style_parts:
    crate::sdk::RepeatedPart<crate::parts::chart_color_style_part::ChartColorStylePart>,
}
