//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2022/11/relationships/FeaturePropertyBag";
pub const PATH_PREFIX: &str = "featurePropertyBag";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct FeaturePropertyBagsPart {
    pub r_id: String,
    pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
    pub rels_path: String,
    pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
    pub inner_path: String,
    pub root_element: crate::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag::FeaturePropertyBags,
}
