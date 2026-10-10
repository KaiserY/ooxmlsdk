//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Debug, ooxmlsdk_derive::SdkPackage)]
pub struct VisioDocument {
  pub(crate) storage: crate::common::SdkPackageStorage,
  pub(crate) open_settings: crate::sdk::OpenSettings,
  pub(crate) root_elements: crate::sdk::PartRootCache,
  #[sdk(package_main(accessor = "visio_document_part"))]
  #[sdk(part_child(kind = "required"))]
  pub(crate) visio_document_part:
    crate::sdk::RequiredPart<crate::parts::visio_document_part::VisioDocumentPart>,
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
}
