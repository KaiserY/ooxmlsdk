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
pub struct WordprocessingDocument {
  pub(crate) storage: crate::common::SdkPackageStorage,
  pub(crate) main_part_id: Option<crate::common::PartId>,
  pub(crate) root_elements: Vec<Option<crate::parts::PartRootElement>>,
  #[sdk(package_main(accessor = "main_document_part"))]
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
    kind = "required"
  ))]
  pub(crate) main_document_part: Option<Box<crate::parts::main_document_part::MainDocumentPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
    kind = "optional"
  ))]
  pub(crate) core_file_properties_part:
    Option<Box<crate::parts::core_file_properties_part::CoreFilePropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
    kind = "optional"
  ))]
  pub(crate) extended_file_properties_part:
    Option<Box<crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
    kind = "optional"
  ))]
  pub(crate) custom_file_properties_part:
    Option<Box<crate::parts::custom_file_properties_part::CustomFilePropertiesPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    kind = "optional"
  ))]
  pub(crate) thumbnail_part: Option<Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin",
    kind = "optional"
  ))]
  pub(crate) digital_signature_origin_part:
    Option<Box<crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart>>,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization",
    kind = "optional"
  ))]
  pub(crate) quick_access_toolbar_customizations_part: Option<
    Box<
      crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    >,
  >,
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility",
    kind = "optional"
  ))]
  pub(crate) ribbon_extensibility_part:
    Option<Box<crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility",
    kind = "optional"
  ))]
  pub(crate) ribbon_and_backstage_customizations_part: Option<
    Box<
      crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    >,
  >,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes",
    kind = "optional"
  ))]
  pub(crate) web_ex_taskpanes_part:
    Option<Box<crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>>,
  #[cfg(feature = "microsoft365")]
  #[sdk(part_child(
    relationship_type = "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels",
    kind = "optional"
  ))]
  pub(crate) label_info_part: Option<Box<crate::parts::label_info_part::LabelInfoPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) data_part_reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) reference_relationships: Vec<crate::common::RelationshipInfo>,
  pub(crate) raw_relationships: Vec<crate::common::RelationshipInfo>,
}
