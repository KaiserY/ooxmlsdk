//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2014/relationships/chartEx";
pub const PATH_PREFIX: &str = "extendedCharts";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct ExtendedChartPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element: crate::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace,
  pub chart_drawing_part:
    Option<std::boxed::Box<crate::parts::chart_drawing_part::ChartDrawingPart>>,
  pub embedded_package_part:
    Option<std::boxed::Box<crate::parts::embedded_package_part::EmbeddedPackagePart>>,
  pub image_parts: Vec<crate::parts::image_part::ImagePart>,
  pub theme_override_part:
    Option<std::boxed::Box<crate::parts::theme_override_part::ThemeOverridePart>>,
  #[cfg(feature = "microsoft365")]
  pub chart_style_parts: Vec<crate::parts::chart_style_part::ChartStylePart>,
  #[cfg(feature = "microsoft365")]
  pub chart_color_style_parts: Vec<crate::parts::chart_color_style_part::ChartColorStylePart>,
}
