//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2006/relationships/wordVbaData";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str = "application/vnd.ms-word.vbaData+xml";
pub const TARGET_NAME: &str = "vbaData";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct VbaDataPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl VbaDataPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_word_2006_wordml::VbaSuppData,
    VbaDataPart,
    as_vba_data_part,
    as_vba_data_part_mut
  );
}
