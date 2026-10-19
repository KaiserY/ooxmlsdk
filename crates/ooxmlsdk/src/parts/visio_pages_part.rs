//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[doc(hidden)]
#[derive(ooxmlsdk_derive::SdkPart)]
#[sdk(part_handle_spec)]
pub struct VisioPagesPartSpec {
  pub(crate) root_element:
    crate::sdk::PartRoot<crate::schemas::schemas_microsoft_com_office_visio_2012_main::Pages>,
  pub(crate) visio_page_parts:
    crate::sdk::RepeatedPart<crate::parts::visio_page_part::VisioPagePart>,
}
pub type VisioPagesPart = crate::sdk::PartHandle<VisioPagesPartSpec>;
