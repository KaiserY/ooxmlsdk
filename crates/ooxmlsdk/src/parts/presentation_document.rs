//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str = "";
pub const PATH_PREFIX: &str = "";
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkPart)]
pub struct PresentationDocument {
  #[sdk(part_content_types)]
  pub content_types: crate::schemas::opc_content_types::Types,
  #[sdk(part_relationships)]
  pub relationships: Option<crate::schemas::opc_relationships::Relationships>,
  #[sdk(part_rels_path)]
  pub rels_path: String,
  #[sdk(part_inner_path)]
  pub inner_path: String,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
    kind = "required"
  ))]
  pub presentation_part: std::boxed::Box<crate::parts::presentation_part::PresentationPart>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
    kind = "optional"
  ))]
  pub core_file_properties_part:
    Option<std::boxed::Box<crate::parts::core_file_properties_part::CoreFilePropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
    kind = "optional"
  ))]
  pub extended_file_properties_part: Option<
    std::boxed::Box<crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart>,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
    kind = "optional"
  ))]
  pub custom_file_properties_part:
    Option<std::boxed::Box<crate::parts::custom_file_properties_part::CustomFilePropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    kind = "optional"
  ))]
  pub thumbnail_part: Option<std::boxed::Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin",
    kind = "optional"
  ))]
  pub digital_signature_origin_part: Option<
    std::boxed::Box<crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart>,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization",
    kind = "optional"
  ))]
  pub quick_access_toolbar_customizations_part: Option<
    std::boxed::Box<
      crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    >,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility",
    kind = "optional"
  ))]
  pub ribbon_extensibility_part:
    Option<std::boxed::Box<crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility",
    kind = "optional"
  ))]
  pub ribbon_and_backstage_customizations_part: Option<
    std::boxed::Box<
      crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    >,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes",
    kind = "optional"
  ))]
  pub web_ex_taskpanes_part:
    Option<std::boxed::Box<crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels",
    kind = "optional"
  ))]
  pub label_info_part: Option<std::boxed::Box<crate::parts::label_info_part::LabelInfoPart>>,
}
