//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes";
pub const PATH_PREFIX: &str = "../drawings";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml";
pub const TARGET_NAME: &str = "drawing";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ChartDrawingPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl ChartDrawingPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes,
    ChartDrawingPart,
    as_chart_drawing_part,
    as_chart_drawing_part_mut
  );
  crate::sdk_part_child_methods! {
      optional chart_part => crate ::parts::chart_part::ChartPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart";
      optional extended_chart_part => crate
      ::parts::extended_chart_part::ExtendedChartPart,
      "http://schemas.microsoft.com/office/2014/relationships/chartEx"; repeated
      image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
  }
}
