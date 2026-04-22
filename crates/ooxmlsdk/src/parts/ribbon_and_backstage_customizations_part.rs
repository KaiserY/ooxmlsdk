//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility";
pub const PATH_PREFIX: &str = "customUI";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct RibbonAndBackstageCustomizationsPart {
  pub r_id: String,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub root_element: crate::schemas::schemas_microsoft_com_office_2009_07_customui::CustomUi,
  pub image_parts: Vec<crate::parts::image_part::ImagePart>,
}
