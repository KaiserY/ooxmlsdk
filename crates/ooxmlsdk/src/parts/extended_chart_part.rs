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
  #[sdk(part_rid)]
  pub r_id: String,
  #[sdk(part_relationships)]
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  #[sdk(part_rels_path)]
  pub rels_path: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_root)]
  pub root_element: crate::schemas::schemas_microsoft_com_office_drawing_2014_chartex::ChartSpace,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes",
    kind = "optional"
  ))]
  pub chart_drawing_part:
    Option<std::boxed::Box<crate::parts::chart_drawing_part::ChartDrawingPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    kind = "optional"
  ))]
  pub embedded_package_part:
    Option<std::boxed::Box<crate::parts::embedded_package_part::EmbeddedPackagePart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    kind = "repeated"
  ))]
  pub image_parts: Vec<crate::parts::image_part::ImagePart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride",
    kind = "optional"
  ))]
  pub theme_override_part:
    Option<std::boxed::Box<crate::parts::theme_override_part::ThemeOverridePart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/chartStyle",
    kind = "repeated"
  ))]
  pub chart_style_parts: Vec<crate::parts::chart_style_part::ChartStylePart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle",
    kind = "repeated"
  ))]
  pub chart_color_style_parts: Vec<crate::parts::chart_color_style_part::ChartColorStylePart>,
}
