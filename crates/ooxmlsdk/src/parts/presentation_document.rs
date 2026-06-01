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
  pub(crate) open_settings: crate::sdk::OpenSettings,
  pub(crate) main_part_id: Option<crate::common::PartId>,
  pub(crate) root_elements: Vec<Option<crate::parts::PartRootElement>>,
  #[sdk(package_main(accessor = "presentation_part"))]
  #[sdk(part_child(relationship_type = RelationshipOfficeDocument, kind = "required"))]
  pub(crate) presentation_part: Option<Box<crate::parts::presentation_part::PresentationPart>>,
  #[sdk(part_child(relationship_type = RelationshipCoreProperties, kind = "optional"))]
  pub(crate) core_file_properties_part:
    Option<Box<crate::parts::core_file_properties_part::CoreFilePropertiesPart>>,
  #[sdk(
        part_child(relationship_type = RelationshipExtendedProperties, kind = "optional")
    )]
  pub(crate) extended_file_properties_part:
    Option<Box<crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart>>,
  #[sdk(
        part_child(relationship_type = RelationshipCustomProperties, kind = "optional")
    )]
  pub(crate) custom_file_properties_part:
    Option<Box<crate::parts::custom_file_properties_part::CustomFilePropertiesPart>>,
  #[sdk(part_child(relationship_type = RelationshipThumbnail, kind = "optional"))]
  pub(crate) thumbnail_part: Option<Box<crate::parts::thumbnail_part::ThumbnailPart>>,
  #[sdk(part_child(relationship_type = RelationshipOrigin, kind = "optional"))]
  pub(crate) digital_signature_origin_part:
    Option<Box<crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart>>,
  #[sdk(
        part_child(relationship_type = RelationshipUserCustomization, kind = "optional")
    )]
  pub(crate) quick_access_toolbar_customizations_part: Option<
    Box<
      crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
    >,
  >,
  #[sdk(part_child(relationship_type = RelationshipExtensibility2, kind = "optional"))]
  pub(crate) ribbon_extensibility_part:
    Option<Box<crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart>>,
  #[sdk(part_child(relationship_type = RelationshipExtensibility, kind = "optional"))]
  pub(crate) ribbon_and_backstage_customizations_part: Option<
    Box<
      crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
    >,
  >,
  #[sdk(
        part_child(
            relationship_type = RelationshipWebextensiontaskpanes,
            kind = "optional"
        )
    )]
  pub(crate) web_ex_taskpanes_part:
    Option<Box<crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart>>,
  #[sdk(
        part_child(
            relationship_type = RelationshipClassificationlabels,
            kind = "optional"
        )
    )]
  pub(crate) label_info_part: Option<Box<crate::parts::label_info_part::LabelInfoPart>>,
  pub(crate) fallback_parts: Vec<crate::parts::PartRef>,
  pub(crate) relationship_order: Vec<crate::sdk::RelationshipModelEntry>,
  pub(crate) modeled_relationships: Vec<crate::common::RelationshipInfo>,
}
