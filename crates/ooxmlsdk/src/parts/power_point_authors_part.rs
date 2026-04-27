//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2018/10/relationships/authors";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str = "application/vnd.ms-powerpoint.authors+xml";
pub const TARGET_NAME: &str = "authors";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct PowerPointAuthorsPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl PowerPointAuthorsPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_powerpoint_2018_8_main::AuthorList,
    PowerPointAuthorsPart,
    as_power_point_authors_part,
    as_power_point_authors_part_mut
  );
}
