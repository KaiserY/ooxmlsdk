//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2007/relationships/customDataProps";
pub const PATH_PREFIX: &str = "customData";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.customDataProperties+xml";
pub const TARGET_NAME: &str = "customDataProps";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CustomDataPropertiesPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl CustomDataPropertiesPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem,
    CustomDataPropertiesPart,
    as_custom_data_properties_part,
    as_custom_data_properties_part_mut
  );
  pub fn custom_data_part<P: crate::sdk::SdkPackage>(
    &self,
    package: &P,
  ) -> Option<crate::parts::custom_data_part::CustomDataPart> {
    <Self as crate::sdk::SdkPart>::child_part_by_relationship_type::<
      P,
      crate::parts::custom_data_part::CustomDataPart,
    >(
      self,
      package,
      "http://schemas.microsoft.com/office/2007/relationships/customData",
    )
  }
}
