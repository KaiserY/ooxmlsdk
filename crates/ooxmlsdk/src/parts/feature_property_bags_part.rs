//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag";
pub const PATH_PREFIX: &str = "featurePropertyBag";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.featurepropertybag+xml";
pub const TARGET_NAME: &str = "featurePropertyBag";
pub const EXTENSION: &str = "";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct FeaturePropertyBagsPart {
    pub(crate) id: crate::common::PartId,
    #[sdk(part_root(accessor = "as_feature_property_bags_part"))]
    pub(crate) root_element: crate::sdk::PartRoot<
        crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag::FeaturePropertyBags,
    >,
}
