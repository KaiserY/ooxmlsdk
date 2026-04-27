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
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[
  crate::sdk::PartChildDescriptor::new(
    "chart_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
    "crate::parts::chart_part::ChartPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "extended_chart_part",
    "http://schemas.microsoft.com/office/2014/relationships/chartEx",
    "crate::parts::extended_chart_part::ExtendedChartPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "image_parts",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    "crate::parts::image_part::ImagePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
];
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
  pub fn chart_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::chart_part::ChartPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::chart_part::ChartPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn extended_chart_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::extended_chart_part::ExtendedChartPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::extended_chart_part::ExtendedChartPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2014/relationships/chartEx",
    )
  }
  pub fn image_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::image_part::ImagePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::image_part::ImagePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    )
  }
}
