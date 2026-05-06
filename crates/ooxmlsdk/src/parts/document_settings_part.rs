//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings";
pub const PATH_PREFIX: &str = ".";
pub const CONTENT_TYPE: &str =
  "application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml";
pub const TARGET_NAME: &str = "settings";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DocumentSettingsPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl DocumentSettingsPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings,
    DocumentSettingsPart,
    as_document_settings_part,
    as_document_settings_part_mut
  );
  crate::sdk_part_child_methods! {
      optional mail_merge_recipient_data_part => crate
      ::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData";
      repeated image_parts => crate ::parts::image_part::ImagePart,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
  }
}
