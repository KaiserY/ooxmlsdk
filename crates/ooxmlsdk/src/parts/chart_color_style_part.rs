//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str = "application/vnd.ms-office.chartcolorstyle+xml";
pub const TARGET_NAME: &str = "colors";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ChartColorStylePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl ChartColorStylePart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_drawing_2012_chart_style::ColorStyle,
    ChartColorStylePart,
    as_chart_color_style_part,
    as_chart_color_style_part_mut
  );
}
