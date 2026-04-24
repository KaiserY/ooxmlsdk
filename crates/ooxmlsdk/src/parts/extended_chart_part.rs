//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2014/relationships/chartEx";
pub const PATH_PREFIX: &str = "extendedCharts";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ExtendedChartPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_extended_chart_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes",
    kind = "optional"
  ))]
  pub(crate) chart_drawing_part:
    crate::sdk::PartChild<crate::parts::chart_drawing_part::ChartDrawingPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    kind = "optional"
  ))]
  pub(crate) embedded_package_part:
    crate::sdk::PartChild<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    kind = "repeated"
  ))]
  pub(crate) image_parts: crate::sdk::PartChild<crate::parts::image_part::ImagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride",
    kind = "optional"
  ))]
  pub(crate) theme_override_part:
    crate::sdk::PartChild<crate::parts::theme_override_part::ThemeOverridePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/chartStyle",
    kind = "repeated"
  ))]
  pub(crate) chart_style_parts:
    crate::sdk::PartChild<crate::parts::chart_style_part::ChartStylePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle",
    kind = "repeated"
  ))]
  pub(crate) chart_color_style_parts:
    crate::sdk::PartChild<crate::parts::chart_color_style_part::ChartColorStylePart>,
}
