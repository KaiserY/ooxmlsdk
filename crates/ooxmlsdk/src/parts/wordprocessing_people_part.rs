//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str = "http://schemas.microsoft.com/office/2011/relationships/people";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.people+xml";
pub const TARGET_NAME: &str = "people";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct WordprocessingPeoplePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl WordprocessingPeoplePart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_word_2012_wordml::People,
    WordprocessingPeoplePart,
    as_wordprocessing_people_part,
    as_wordprocessing_people_part_mut
  );
}
