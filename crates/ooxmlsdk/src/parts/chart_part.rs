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
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[
  crate::sdk::PartChildDescriptor::new(
    "chart_drawing_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes",
    "crate::parts::chart_drawing_part::ChartDrawingPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "embedded_package_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    "crate::parts::embedded_package_part::EmbeddedPackagePart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "image_parts",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    "crate::parts::image_part::ImagePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "theme_override_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride",
    "crate::parts::theme_override_part::ThemeOverridePart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "chart_style_parts",
    "http://schemas.microsoft.com/office/2011/relationships/chartStyle",
    "crate::parts::chart_style_part::ChartStylePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
  crate::sdk::PartChildDescriptor::new(
    "chart_color_style_parts",
    "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle",
    "crate::parts::chart_color_style_part::ChartColorStylePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
];
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
  pub fn chart_drawing_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes",
    )
  }
  pub fn chart_drawing_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::chart_drawing_part::ChartDrawingPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::chart_drawing_part::ChartDrawingPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chartUserShapes",
    )
  }
  pub fn embedded_package_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    )
  }
  pub fn embedded_package_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::embedded_package_part::EmbeddedPackagePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::embedded_package_part::EmbeddedPackagePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
    )
  }
  pub fn image_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
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
  pub fn theme_override_part_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride",
    )
  }
  pub fn theme_override_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::theme_override_part::ThemeOverridePart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::theme_override_part::ThemeOverridePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/themeOverride",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn chart_style_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2011/relationships/chartStyle",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn chart_style_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::chart_style_part::ChartStylePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::chart_style_part::ChartStylePart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2011/relationships/chartStyle",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn chart_color_style_parts_relationships<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::common::RelationshipRef<'a>> + 'a {
    <Self as crate::sdk::SdkPart>::child_relationships_by_type(
      self,
      package,
      "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle",
    )
  }
  #[cfg(feature = "microsoft365")]
  pub fn chart_color_style_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::chart_color_style_part::ChartColorStylePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::chart_color_style_part::ChartColorStylePart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2011/relationships/chartColorStyle",
    )
  }
}
