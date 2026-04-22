//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2017/10/relationships/person";
pub const PATH_PREFIX: &str = "persons";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct WorkbookPersonPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element:
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments::PersonList,
}
