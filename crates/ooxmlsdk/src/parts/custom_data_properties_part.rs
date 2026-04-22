//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2007/relationships/customDataProps";
pub const PATH_PREFIX: &str = "customData";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct CustomDataPropertiesPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element:
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::DatastoreItem,
  #[cfg(feature = "microsoft365")]
  pub custom_data_part: Option<std::boxed::Box<crate::parts::custom_data_part::CustomDataPart>>,
}
