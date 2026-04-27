//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2017/06/relationships/rdSupportingPropertyBagStructure";
pub const PATH_PREFIX: &str = "richData";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.rdsupportingpropertybagstructure+xml";
pub const TARGET_NAME: &str = "rdsupportingpropertybagstructure";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct RdSupportingPropertyBagStructurePart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl RdSupportingPropertyBagStructurePart {
  crate::sdk_part_root_methods!(
        crate
        ::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2::SupportingPropertyBagStructures,
        RdSupportingPropertyBagStructurePart,
        as_rd_supporting_property_bag_structure_part,
        as_rd_supporting_property_bag_structure_part_mut
    );
}
