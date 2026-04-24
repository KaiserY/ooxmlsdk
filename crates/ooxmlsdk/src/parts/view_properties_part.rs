//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps";
pub const PATH_PREFIX: &str = ".";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ViewPropertiesPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_view_properties_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ViewProperties,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide"
  ))]
  pub(crate) slide_parts: crate::sdk::RepeatedPart<crate::parts::slide_part::SlidePart>,
}
