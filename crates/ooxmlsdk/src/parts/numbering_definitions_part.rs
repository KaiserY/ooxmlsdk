//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml";
pub const TARGET_NAME: &str = "numbering";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct NumberingDefinitionsPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl NumberingDefinitionsPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Numbering,
    NumberingDefinitionsPart,
    as_numbering_definitions_part,
    as_numbering_definitions_part_mut
  );
  crate::sdk_part_child_methods! {
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
  }
}
