//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Eq, Hash, PartialEq, ooxmlsdk_derive::SdkPart)]
pub struct FontTablePart {
  pub(crate) key: crate::common::PartKey,
  pub(crate) root_element: crate::sdk::PartRoot<
    crate::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main::Fonts,
  >,
  pub(crate) font_parts: crate::sdk::RepeatedPart<crate::parts::font_part::FontPart>,
}
