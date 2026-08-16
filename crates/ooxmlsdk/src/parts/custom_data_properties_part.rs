//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct CustomDataPropertiesPartSpec {
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem,
  >,
  pub(crate) custom_data_part:
    crate::sdk::OptionalPart<crate::parts::custom_data_part::CustomDataPart>,
}
pub type CustomDataPropertiesPart = crate::sdk::PartHandle<CustomDataPropertiesPartSpec>;
