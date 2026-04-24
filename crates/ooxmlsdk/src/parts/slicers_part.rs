//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str = "http://schemas.microsoft.com/office/2007/relationships/slicer";
pub const PATH_PREFIX: &str = "../slicers";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.slicer+xml";
pub const TARGET_NAME: &str = "slicer";
pub const EXTENSION: &str = "";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct SlicersPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_slicers_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::Slicers,
  >,
}
