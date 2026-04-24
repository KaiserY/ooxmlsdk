//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels";
pub const PATH_PREFIX: &str = "docMetadata";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct LabelInfoPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_label_info_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList,
  >,
}
