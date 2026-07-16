//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct DocumentSettingsPart {
  pub(crate) id: crate::common::PartId,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Settings,
  >,
  pub(crate) mail_merge_recipient_data_part: crate::sdk::OptionalPart<
    crate::parts::mail_merge_recipient_data_part::MailMergeRecipientDataPart,
  >,
  pub(crate) image_parts: crate::sdk::RepeatedPart<crate::parts::image_part::ImagePart>,
}
