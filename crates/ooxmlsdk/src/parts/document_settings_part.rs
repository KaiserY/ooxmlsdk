//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct DocumentSettingsPartSpec {
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings,
  >,
  pub(crate) mail_merge_recipient_data_part: crate::sdk::OptionalPart<
    crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
  >,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
pub type DocumentSettingsPart = crate::sdk::PartHandle<DocumentSettingsPartSpec>;
