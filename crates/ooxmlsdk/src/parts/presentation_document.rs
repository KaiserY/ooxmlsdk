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
  pub(crate) presentation_part: Option<Box<crate::parts::presentation_part::PresentationPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
  ))]
  pub(crate) core_file_properties_part:
    Option<Box<crate::parts::core_file_properties_part::CoreFilePropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties"
  ))]
  pub(crate) extended_file_properties_part:
    Option<Box<crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties"
  ))]
  pub(crate) custom_file_properties_part:
    Option<Box<crate::parts::custom_file_properties_part::CustomFilePropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
  ))]
  pub(crate) thumbnail_part: Option<Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin"
  ))]
  pub(crate) digital_signature_origin_part:
    Option<Box<crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization"
  ))]
  pub(crate) quick_access_toolbar_customizations_part: Option<
    Box<
      crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    >,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility"
  ))]
  pub(crate) ribbon_extensibility_part:
    Option<Box<crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility"
  ))]
  pub(crate) ribbon_and_backstage_customizations_part: Option<
    Box<
      crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    >,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes"
  ))]
  pub(crate) web_ex_taskpanes_part:
    Option<Box<crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels"
  ))]
  pub(crate) label_info_part: Option<Box<crate::parts::label_info_part::LabelInfoPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<Box<str>>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
impl PresentationDocument {
  pub const GENERATED_CHILD_DESCRIPTORS: &'static [crate::sdk::PartChildDescriptor] = &[
    crate::sdk::PartChildDescriptor::new(
      "presentation_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
      "crate::parts::presentation_part::PresentationPart",
      crate::sdk::PartChildCardinality::Required,
    ),
    crate::sdk::PartChildDescriptor::new(
      "core_file_properties_part",
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
      "crate::parts::core_file_properties_part::CoreFilePropertiesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "extended_file_properties_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
      "crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "custom_file_properties_part",
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
      "crate::parts::custom_file_properties_part::CustomFilePropertiesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "thumbnail_part",
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
      "crate::parts::thumbnail_part::ThumbnailPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "digital_signature_origin_part",
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin",
      "crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "quick_access_toolbar_customizations_part",
      "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization",
      "crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    crate::sdk::PartChildDescriptor::new(
      "ribbon_extensibility_part",
      "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility",
      "crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "ribbon_and_backstage_customizations_part",
      "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility",
      "crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "web_ex_taskpanes_part",
      "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes",
      "crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
    #[cfg(feature = "microsoft365")]
    crate::sdk::PartChildDescriptor::new(
      "label_info_part",
      "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels",
      "crate::parts::label_info_part::LabelInfoPart",
      crate::sdk::PartChildCardinality::Optional,
    ),
  ];
}
