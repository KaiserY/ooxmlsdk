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
  #[sdk(part_root(accessor = "as_label_info_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_2020_mip_label_metadata::ClassificationLabelList,
  >,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
