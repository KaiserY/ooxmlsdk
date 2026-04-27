//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml";
pub const TARGET_NAME: &str = "viewProps";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] =
  &[crate::sdk::PartChildDescriptor::new(
    "slide_parts",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
    "crate::parts::slide_part::SlidePart",
    crate::sdk::PartChildCardinality::Repeated,
  )];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ViewPropertiesPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl ViewPropertiesPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ViewProperties,
    ViewPropertiesPart,
    as_view_properties_part,
    as_view_properties_part_mut
  );
  pub fn slide_parts<'a, P: crate::sdk::SdkPackage>(
    &'a self,
    package: &'a P,
  ) -> impl Iterator<Item = crate::parts::slide_part::SlidePart> + 'a {
    <Self as crate::sdk::SdkPart>::child_parts_by_relationship_type::<
      P,
      crate::parts::slide_part::SlidePart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide",
    )
  }
}
