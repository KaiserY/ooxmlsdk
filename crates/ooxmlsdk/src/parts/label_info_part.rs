//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels";
pub const PATH_PREFIX: &str = "docMetadata";
pub const CONTENT_TYPE: &str = "application/vnd.ms-office.classificationlabels+xml";
pub const TARGET_NAME: &str = "LabelInfo";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct LabelInfoPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl LabelInfoPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList,
    LabelInfoPart,
    as_label_info_part,
    as_label_info_part_mut
  );
}
