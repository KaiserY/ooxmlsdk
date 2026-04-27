//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str =
  "http://schemas.openxmlformats.org/officeDocument/2006/relationships/ctrlProp";
pub const PATH_PREFIX: &str = "../ctrlProps";
pub const CONTENT_TYPE: &str = "application/vnd.ms-excel.controlproperties+xml";
pub const TARGET_NAME: &str = "ctrlProp";
pub const EXTENSION: &str = "";
pub const CHILD_DESCRIPTORS: &[crate::sdk::PartChildDescriptor] = &[];
#[derive(Clone, Debug, Eq, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct ControlPropertiesPart {
  pub(crate) relationship_id: Option<String>,
  pub(crate) id: crate::common::PartId,
}
impl ControlPropertiesPart {
  crate::sdk_part_root_methods!(
    crate::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main::FormControlProperties,
    ControlPropertiesPart,
    as_control_properties_part,
    as_control_properties_part_mut
  );
}
