//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes";
pub const PATH_PREFIX: &str = "../drawings";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct ChartDrawingPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element: crate::schemas::schemas_openxmlformats_org_drawingml_2006_chart::UserShapes,
  pub chart_part: Option<std::boxed::Box<crate::parts::chart_part::ChartPart>>,
  #[cfg(feature = "microsoft365")]
  pub extended_chart_part:
    Option<std::boxed::Box<crate::parts::extended_chart_part::ExtendedChartPart>>,
  pub image_parts: Vec<crate::parts::image_part::ImagePart>,
}
