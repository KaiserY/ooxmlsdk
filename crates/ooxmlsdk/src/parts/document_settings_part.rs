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
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[
  crate::sdk::PartChildDescriptor::new(
    "mail_merge_recipient_data_part",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData",
    "crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart",
    crate::sdk::PartChildCardinality::Optional,
  ),
  crate::sdk::PartChildDescriptor::new(
    "image_parts",
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
    "crate::parts::image_part::ImagePart",
    crate::sdk::PartChildCardinality::Repeated,
  ),
];
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
  pub fn mail_merge_recipient_data_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
    >(
      self,
      package,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/recipientData",
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
}
