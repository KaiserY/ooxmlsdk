//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart";
pub const PATH_PREFIX: &str = "charts";
pub const CONTENT_TYPE: &str = "application/vnd.openxmlformats-officedocument.drawingml.chart+xml";
pub const TARGET_NAME: &str = "chart";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ChartPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl ChartPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace,
    ChartPart,
    as_chart_part,
    as_chart_part_mut
  );
  crate::sdk_part_child_methods! {
      optional chart_drawing_part => crate
      ::parts::chart_drawing_part::ChartDrawingPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes";
      optional embedded_package_part => crate
      ::parts::embedded_package_part::EmbeddedPackagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package";
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
      optional theme_override_part => crate
      ::parts::theme_override_part::ThemeOverridePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride";
      repeated chart_style_parts => crate ::parts::chart_style_part::ChartStylePart,
      "http://schemas.microsoft.com/office/2011/relationships/chartStyle"; repeated
      chart_color_style_parts => crate
      ::parts::chart_color_style_part::ChartColorStylePart,
      "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle";
  }
}
