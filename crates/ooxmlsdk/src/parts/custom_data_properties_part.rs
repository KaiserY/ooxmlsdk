//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2007/relationships/customDataProps";
pub const PATH_PREFIX: &str = "customData";
#[derive(Clone, Copy, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct CustomDataPropertiesPart {
  pub(crate) id: crate::common::PartId,
  #[sdk(part_root(accessor = "as_custom_data_properties_part"))]
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/customData",
    kind = "optional"
  ))]
  pub(crate) custom_data_part:
    crate::sdk::PartChild<crate::parts::custom_data_part::CustomDataPart>,
}
