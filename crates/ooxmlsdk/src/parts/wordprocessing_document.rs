//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str = "";
pub const PATH_PREFIX: &str = "";
#[derive(Clone, Debug, ooxmlsdk_derive::SdkPackage)]
pub struct WordprocessingDocument {
  pub(crate) storage: crate::common::SdkPackageStorage,
  pub(crate) main_part_id: Option<crate::common::PartId>,
  pub(crate) root_elements: Vec<Option<crate::parts::PartRootElement>>,
  #[sdk(package_main(accessor = "main_document_part"))]
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
    kind = "required"
  ))]
  #[allow(dead_code)]
  pub(crate) main_document_part:
    crate::sdk::PartChild<crate::parts::main_document_part::MainDocumentPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
    kind = "optional"
  ))]
  #[allow(dead_code)]
  pub(crate) core_file_properties_part:
    crate::sdk::PartChild<crate::parts::core_file_properties_part::CoreFilePropertiesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
    kind = "optional"
  ))]
  #[allow(dead_code)]
  pub(crate) extended_file_properties_part:
    crate::sdk::PartChild<crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
    kind = "optional"
  ))]
  #[allow(dead_code)]
  pub(crate) custom_file_properties_part:
    crate::sdk::PartChild<crate::parts::custom_file_properties_part::CustomFilePropertiesPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    kind = "optional"
  ))]
  #[allow(dead_code)]
  pub(crate) thumbnail_part: crate::sdk::PartChild<crate::parts::thumbnail_part::ThumbnailPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin",
    kind = "optional"
  ))]
  #[allow(dead_code)]
  pub(crate) digital_signature_origin_part:
    crate::sdk::PartChild<crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization",
    kind = "optional"
  ))]
  #[allow(dead_code)]
  pub(crate) quick_access_toolbar_customizations_part: crate::sdk::PartChild<
    crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility",
    kind = "optional"
  ))]
  #[allow(dead_code)]
  pub(crate) ribbon_extensibility_part:
    crate::sdk::PartChild<crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility",
    kind = "optional"
  ))]
  #[allow(dead_code)]
  pub(crate) ribbon_and_backstage_customizations_part: crate::sdk::PartChild<
    crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes",
    kind = "optional"
  ))]
  #[allow(dead_code)]
  pub(crate) web_ex_taskpanes_part:
    crate::sdk::PartChild<crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels",
    kind = "optional"
  ))]
  #[allow(dead_code)]
  pub(crate) label_info_part: crate::sdk::PartChild<crate::parts::label_info_part::LabelInfoPart>,
}
