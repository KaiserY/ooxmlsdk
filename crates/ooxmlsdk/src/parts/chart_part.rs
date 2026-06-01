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
  #[sdk(part_root(accessor = "as_chart_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::ChartSpace,
  >,
  #[sdk(part_child(relationship_type = RelationshipChartUserShapes))]
  pub(crate) chart_drawing_part:
    crate::sdk::OptionalPart<crate::parts::chart_drawing_part::ChartDrawingPart>,
  #[sdk(part_child(relationship_type = RelationshipPackage))]
  pub(crate) embedded_package_part:
    crate::sdk::OptionalPart<crate::parts::embedded_package_part::EmbeddedPackagePart>,
  #[sdk(part_child(relationship_type = RelationshipImage))]
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
  #[sdk(part_child(relationship_type = RelationshipThemeOverride))]
  pub(crate) theme_override_part:
    crate::sdk::OptionalPart<crate::parts::theme_override_part::ThemeOverridePart>,
  #[sdk(part_child(relationship_type = RelationshipChartStyle))]
  pub(crate) chart_style_parts:
    crate::sdk::RepeatedPart<crate::parts::chart_style_part::ChartStylePart>,
  #[sdk(part_child(relationship_type = RelationshipChartColorStyle))]
  pub(crate) chart_color_style_parts:
    crate::sdk::RepeatedPart<crate::parts::chart_color_style_part::ChartColorStylePart>,
}
