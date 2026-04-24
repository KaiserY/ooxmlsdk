//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str = "";
pub const PATH_PREFIX: &str = "";
pub const CONTENT_TYPE: &str = "";
pub const TARGET_NAME: &str = "";
pub const EXTENSION: &str = "";
#[derive(Clone, Debug, ooxmlsdk_derive::SdkPackage)]
pub struct PresentationDocument {
  pub(crate) storage: crate::common::SdkPackageStorage,
  pub(crate) main_part_id: Option<crate::common::PartId>,
  pub(crate) root_elements: Vec<Option<crate::parts::PartRootElement>>,
  #[sdk(package_main(accessor = "presentation_part"))]
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
  ))]
  pub(crate) presentation_part:
    crate::sdk::RequiredPart<crate::parts::presentation_part::PresentationPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
  ))]
  pub(crate) core_file_properties_part:
    crate::sdk::OptionalPart<crate::parts::core_file_properties_part::CoreFilePropertiesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties"
  ))]
  pub(crate) extended_file_properties_part: crate::sdk::OptionalPart<
    crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties"
  ))]
  pub(crate) custom_file_properties_part:
    crate::sdk::OptionalPart<crate::parts::custom_file_properties_part::CustomFilePropertiesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
  ))]
  pub(crate) thumbnail_part: crate::sdk::OptionalPart<crate::parts::thumbnail_part::ThumbnailPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin"
  ))]
  pub(crate) digital_signature_origin_part: crate::sdk::OptionalPart<
    crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization"
  ))]
  pub(crate) quick_access_toolbar_customizations_part: crate::sdk::OptionalPart<
    crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility"
  ))]
  pub(crate) ribbon_extensibility_part:
    crate::sdk::OptionalPart<crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility"
  ))]
  pub(crate) ribbon_and_backstage_customizations_part: crate::sdk::OptionalPart<
    crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes"
  ))]
  pub(crate) web_ex_taskpanes_part:
    crate::sdk::OptionalPart<crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels"
  ))]
  pub(crate) label_info_part:
    crate::sdk::OptionalPart<crate::parts::label_info_part::LabelInfoPart>,
}
