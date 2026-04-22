//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str = "";
pub const PATH_PREFIX: &str = "";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct WordprocessingDocument {
  pub content_types: crate::schemas::opc_content_types::Types,
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  pub rels_path: String,
  pub extended_parts: Vec<crate::common::extended_part::ExtendedPart>,
  pub inner_path: String,
  pub main_document_part: std::boxed::Box<crate::parts::main_document_part::MainDocumentPart>,
  pub core_file_properties_part:
    Option<std::boxed::Box<crate::parts::core_file_properties_part::CoreFilePropertiesPart>>,
  pub extended_file_properties_part: Option<
    std::boxed::Box<crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart>,
  >,
  pub custom_file_properties_part:
    Option<std::boxed::Box<crate::parts::custom_file_properties_part::CustomFilePropertiesPart>>,
  pub thumbnail_part: Option<std::boxed::Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  pub digital_signature_origin_part: Option<
    std::boxed::Box<crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart>,
  >,
  pub quick_access_toolbar_customizations_part: Option<
    std::boxed::Box<
      crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    >,
  >,
  pub ribbon_extensibility_part:
    Option<std::boxed::Box<crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>>,
  #[cfg(feature = "microsoft365")]
  pub ribbon_and_backstage_customizations_part: Option<
    std::boxed::Box<
      crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    >,
  >,
  #[cfg(feature = "microsoft365")]
  pub web_ex_taskpanes_part:
    Option<std::boxed::Box<crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>>,
  #[cfg(feature = "microsoft365")]
  pub label_info_part: Option<std::boxed::Box<crate::parts::label_info_part::LabelInfoPart>>,
}
