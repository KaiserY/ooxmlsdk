//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2014/relationships/chartEx";
pub const PATH_PREFIX: &str = "extendedCharts";
pub const CONTENT_TYPE: &str = "application/vnd.ms-office.chartex+xml";
pub const TARGET_NAME: &str = "chart";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ExtendedChartPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl ExtendedChartPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace,
    ExtendedChartPart,
    as_extended_chart_part,
    as_extended_chart_part_mut
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
