//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct VisioMastersPartSpec {
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::schemas_microsoft_com_office_visio_2012_main::Masters>,
  pub(crate) visio_master_parts:
    crate::sdk::RepeatedPart<crate::parts::visio_master_part::VisioMasterPart>,
}
pub type VisioMastersPart = crate::sdk::PartHandle<VisioMastersPartSpec>;
