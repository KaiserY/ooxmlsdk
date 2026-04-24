//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties";
pub const PATH_PREFIX: &str = "docProps";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ExtendedFilePropertiesPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_extended_file_properties_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_office_document_2006_extended_properties::Properties,
  >,
}
