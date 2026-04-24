//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

pub const RELATIONSHIP_TYPE: &str = "";
pub const PATH_PREFIX: &str = "";
#[derive(Clone, Debug, ooxmlsdk_derive::SdkPackage)]
pub struct SpreadsheetDocument {
  pub(crate) storage: crate::common::SdkPackageStorage,
  pub(crate) main_part_id: Option<crate::common::PartId>,
  pub(crate) root_elements: Vec<Option<crate::parts::PartRootElement>>,
}
impl SpreadsheetDocument {
  pub const MAIN_PART_RELATIONSHIP_TYPE: &'static str =
    <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartHandle>::RELATIONSHIP_TYPE;
  pub fn workbook_part(
    &self,
  ) -> Result<crate::parts::workbook_part::WorkbookPart, crate::common::SdkError> {
    self
      .main_part_id
      .map(<crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartHandle>::from_part_id)
      .ok_or_else(|| {
        crate::common::SdkError::CommonError(
          concat!("missing main part for ", stringify!(SpreadsheetDocument)).to_string(),
        )
      })
  }
}
impl SpreadsheetDocument {
  pub fn parts(&self) -> impl Iterator<Item = crate::parts::IdPartPair<'_>> + '_ {
    self.relationships().iter().filter_map(|relationship| {
      let part = Self::part_ref_from_relationship(relationship)?;
      Some(crate::parts::IdPartPair::new(relationship.id(), part))
    })
  }
  pub fn get_part_by_id(&self, relationship_id: &str) -> Option<crate::parts::PartRef> {
    let relationship = self.relationships().get(relationship_id)?;
    Self::part_ref_from_relationship(relationship)
  }
  fn part_ref_from_relationship(
    relationship: &crate::common::RelationshipInfo,
  ) -> Option<crate::parts::PartRef> {
    let part_id = relationship.target_part_id()?;
    let relationship_type = relationship.relationship_type();
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
    ) {
      return Some(crate::parts::PartRef::WorkbookPart(
        <crate::parts::workbook_part::WorkbookPart as crate::sdk::SdkPartHandle>::from_part_id(
          part_id,
        ),
      ));
    }
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
    ) {
      return Some(
                crate::parts::PartRef::CoreFilePropertiesPart(
                    <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id(
                        part_id,
                    ),
                ),
            );
    }
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
    ) {
      return Some(
                crate::parts::PartRef::ExtendedFilePropertiesPart(
                    <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id(
                        part_id,
                    ),
                ),
            );
    }
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
    ) {
      return Some(
                crate::parts::PartRef::CustomFilePropertiesPart(
                    <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id(
                        part_id,
                    ),
                ),
            );
    }
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
    ) {
      return Some(crate::parts::PartRef::ThumbnailPart(
        <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartHandle>::from_part_id(
          part_id,
        ),
      ));
    }
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin",
    ) {
      return Some(
                crate::parts::PartRef::DigitalSignatureOriginPart(
                    <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartHandle>::from_part_id(
                        part_id,
                    ),
                ),
            );
    }
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization",
    ) {
      return Some(
                crate::parts::PartRef::QuickAccessToolbarCustomizationsPart(
                    <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartHandle>::from_part_id(
                        part_id,
                    ),
                ),
            );
    }
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility",
    ) {
      return Some(
                crate::parts::PartRef::RibbonExtensibilityPart(
                    <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartHandle>::from_part_id(
                        part_id,
                    ),
                ),
            );
    }
    #[cfg(feature = "microsoft365")]
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility",
    ) {
      return Some(
                crate::parts::PartRef::RibbonAndBackstageCustomizationsPart(
                    <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartHandle>::from_part_id(
                        part_id,
                    ),
                ),
            );
    }
    #[cfg(feature = "microsoft365")]
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes",
    ) {
      return Some(
                crate::parts::PartRef::WebExTaskpanesPart(
                    <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartHandle>::from_part_id(
                        part_id,
                    ),
                ),
            );
    }
    #[cfg(feature = "microsoft365")]
    if crate::common::relationship_type_matches(
      relationship_type,
      "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels",
    ) {
      return Some(crate::parts::PartRef::LabelInfoPart(
        <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartHandle>::from_part_id(
          part_id,
        ),
      ));
    }
    None
  }
  pub fn core_file_properties_part(
    &self,
  ) -> Option<crate::parts::core_file_properties_part::CoreFilePropertiesPart> {
    self.relationships()
            .iter()
            .find_map(|relationship: &crate::common::RelationshipInfo| {
                if crate::common::relationship_type_matches(
                    relationship.relationship_type(),
                    "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
                ) {
                    relationship
                        .target_part_id()
                        .map(
                            <crate::parts::core_file_properties_part::CoreFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id,
                        )
                } else {
                    None
                }
            })
  }
  pub fn extended_file_properties_part(
    &self,
  ) -> Option<crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart> {
    self.relationships()
            .iter()
            .find_map(|relationship: &crate::common::RelationshipInfo| {
                if crate::common::relationship_type_matches(
                    relationship.relationship_type(),
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
                ) {
                    relationship
                        .target_part_id()
                        .map(
                            <crate::parts::extended_file_properties_part::ExtendedFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id,
                        )
                } else {
                    None
                }
            })
  }
  pub fn custom_file_properties_part(
    &self,
  ) -> Option<crate::parts::custom_file_properties_part::CustomFilePropertiesPart> {
    self.relationships()
            .iter()
            .find_map(|relationship: &crate::common::RelationshipInfo| {
                if crate::common::relationship_type_matches(
                    relationship.relationship_type(),
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
                ) {
                    relationship
                        .target_part_id()
                        .map(
                            <crate::parts::custom_file_properties_part::CustomFilePropertiesPart as crate::sdk::SdkPartHandle>::from_part_id,
                        )
                } else {
                    None
                }
            })
  }
  pub fn thumbnail_part(&self) -> Option<crate::parts::thumbnail_part::ThumbnailPart> {
    self.relationships()
            .iter()
            .find_map(|relationship: &crate::common::RelationshipInfo| {
                if crate::common::relationship_type_matches(
                    relationship.relationship_type(),
                    "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail",
                ) {
                    relationship
                        .target_part_id()
                        .map(
                            <crate::parts::thumbnail_part::ThumbnailPart as crate::sdk::SdkPartHandle>::from_part_id,
                        )
                } else {
                    None
                }
            })
  }
  pub fn digital_signature_origin_part(
    &self,
  ) -> Option<crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart> {
    self.relationships()
            .iter()
            .find_map(|relationship: &crate::common::RelationshipInfo| {
                if crate::common::relationship_type_matches(
                    relationship.relationship_type(),
                    "http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin",
                ) {
                    relationship
                        .target_part_id()
                        .map(
                            <crate::parts::digital_signature_origin_part::DigitalSignatureOriginPart as crate::sdk::SdkPartHandle>::from_part_id,
                        )
                } else {
                    None
                }
            })
  }
  pub fn quick_access_toolbar_customizations_part(
    &self,
  ) -> Option<
    crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart,
  > {
    self.relationships()
            .iter()
            .find_map(|relationship: &crate::common::RelationshipInfo| {
                if crate::common::relationship_type_matches(
                    relationship.relationship_type(),
                    "http://schemas.microsoft.com/office/2006/relationships/ui/userCustomization",
                ) {
                    relationship
                        .target_part_id()
                        .map(
                            <crate::parts::quick_access_toolbar_customizations_part::QuickAccessToolbarCustomizationsPart as crate::sdk::SdkPartHandle>::from_part_id,
                        )
                } else {
                    None
                }
            })
  }
  pub fn ribbon_extensibility_part(
    &self,
  ) -> Option<crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart> {
    self.relationships()
            .iter()
            .find_map(|relationship: &crate::common::RelationshipInfo| {
                if crate::common::relationship_type_matches(
                    relationship.relationship_type(),
                    "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility",
                ) {
                    relationship
                        .target_part_id()
                        .map(
                            <crate::parts::ribbon_extensibility_part::RibbonExtensibilityPart as crate::sdk::SdkPartHandle>::from_part_id,
                        )
                } else {
                    None
                }
            })
  }
  #[cfg(feature = "microsoft365")]
  pub fn ribbon_and_backstage_customizations_part(
    &self,
  ) -> Option<
    crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart,
  > {
    self.relationships()
            .iter()
            .find_map(|relationship: &crate::common::RelationshipInfo| {
                if crate::common::relationship_type_matches(
                    relationship.relationship_type(),
                    "http://schemas.microsoft.com/office/2007/relationships/ui/extensibility",
                ) {
                    relationship
                        .target_part_id()
                        .map(
                            <crate::parts::ribbon_and_backstage_customizations_part::RibbonAndBackstageCustomizationsPart as crate::sdk::SdkPartHandle>::from_part_id,
                        )
                } else {
                    None
                }
            })
  }
  #[cfg(feature = "microsoft365")]
  pub fn web_ex_taskpanes_part(
    &self,
  ) -> Option<crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart> {
    self.relationships()
            .iter()
            .find_map(|relationship: &crate::common::RelationshipInfo| {
                if crate::common::relationship_type_matches(
                    relationship.relationship_type(),
                    "http://schemas.microsoft.com/office/2011/relationships/webextensiontaskpanes",
                ) {
                    relationship
                        .target_part_id()
                        .map(
                            <crate::parts::web_ex_taskpanes_part::WebExTaskpanesPart as crate::sdk::SdkPartHandle>::from_part_id,
                        )
                } else {
                    None
                }
            })
  }
  #[cfg(feature = "microsoft365")]
  pub fn label_info_part(&self) -> Option<crate::parts::label_info_part::LabelInfoPart> {
    self.relationships()
            .iter()
            .find_map(|relationship: &crate::common::RelationshipInfo| {
                if crate::common::relationship_type_matches(
                    relationship.relationship_type(),
                    "http://schemas.microsoft.com/office/2020/02/relationships/classificationlabels",
                ) {
                    relationship
                        .target_part_id()
                        .map(
                            <crate::parts::label_info_part::LabelInfoPart as crate::sdk::SdkPartHandle>::from_part_id,
                        )
                } else {
                    None
                }
            })
  }
}
