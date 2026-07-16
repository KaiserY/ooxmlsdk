//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, ooxmlsdk_derive::SdkPackage)]
pub struct PresentationDocument {
  pub(crate) storage: crate::common::SdkPackageStorage,
  pub(crate) open_settings: crate::sdk::OpenSettings,
  pub(crate) root_elements: Vec<Option<crate::parts::PartRootElement>>,
  #[sdk(package_main(accessor = "presentation_part"))]
  #[sdk(part_child(kind = "required"))]
  pub(crate) presentation_part:
    crate::sdk::RequiredPart<crate::parts::presentation_part::PresentationPart>,
  pub(crate) core_file_properties_part:
    crate::sdk::OptionalPart<crate::parts::core_file_properties_part::CoreFilePropertiesPart>,
  pub(crate) extended_file_properties_part: crate::sdk::OptionalPart<
    crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
  >,
  pub(crate) custom_file_properties_part:
    crate::sdk::OptionalPart<crate::parts::custom_file_properties_part::CustomFilePropertiesPart>,
  pub(crate) thumbnail_part: crate::sdk::OptionalPart<crate::parts::thumbnail_part::ThumbnailPart>,
  pub(crate) digital_signature_origin_part: crate::sdk::OptionalPart<
    crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
  >,
  pub(crate) quick_access_toolbar_customizations_part: crate::sdk::OptionalPart<
    crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
  >,
  pub(crate) ribbon_extensibility_part:
    crate::sdk::OptionalPart<crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>,
  pub(crate) ribbon_and_backstage_customizations_part: crate::sdk::OptionalPart<
    crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
  >,
  pub(crate) web_ex_taskpanes_part:
    crate::sdk::OptionalPart<crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>,
  pub(crate) label_info_part:
    crate::sdk::OptionalPart<crate::parts::label_info_part::LabelInfoPart>,
}
