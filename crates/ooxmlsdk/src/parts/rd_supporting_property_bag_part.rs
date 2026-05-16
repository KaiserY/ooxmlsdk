//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBag";
pub const PATH_PREFIX: &str = "richData";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.rdsupportingpropertybag+xml";
pub const TARGET_NAME: &str = "rdsupportingpropertybag";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct RdSupportingPropertyBagPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl RdSupportingPropertyBagPart {
  crate::sdk_part_root_methods!(
        crate
        ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBags,
        RdSupportingPropertyBagPart, as_rd_supporting_property_bag_part,
        as_rd_supporting_property_bag_part_mut
    );
}
