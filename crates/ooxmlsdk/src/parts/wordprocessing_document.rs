//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, ooxmlsdk_derive::SdkPackage)]
pub struct WordprocessingDocument {
  pub(crate) storage: crate::common::SdkPackageStorage,
  pub(crate) open_settings: crate::sdk::OpenSettings,
  pub(crate) main_part_id: Option<crate::common::PartId>,
  pub(crate) root_elements: Vec<Option<crate::parts::PartRootElement>>,
  #[sdk(package_main(accessor = "main_document_part"))]
  #[sdk(part_child(kind = "required"))]
  pub(crate) main_document_part: Option<Box<crate::parts::main_document_part::MainDocumentPart>>,
  pub(crate) core_file_properties_part:
    Option<Box<crate::parts::core_file_properties_part::CoreFilePropertiesPart>>,
  pub(crate) extended_file_properties_part:
    Option<Box<crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart>>,
  pub(crate) custom_file_properties_part:
    Option<Box<crate::parts::custom_file_properties_part::CustomFilePropertiesPart>>,
  pub(crate) thumbnail_part: Option<Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  pub(crate) digital_signature_origin_part:
    Option<Box<crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart>>,
  pub(crate) quick_access_toolbar_customizations_part: Option<
    Box<
      crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    >,
  >,
  pub(crate) ribbon_extensibility_part:
    Option<Box<crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>>,
  pub(crate) ribbon_and_backstage_customizations_part: Option<
    Box<
      crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    >,
  >,
  pub(crate) web_ex_taskpanes_part:
    Option<Box<crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>>,
  pub(crate) label_info_part: Option<Box<crate::parts::label_info_part::LabelInfoPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) modeled_relationships: Vec<crate::common::RelationshipInfo>,
}
