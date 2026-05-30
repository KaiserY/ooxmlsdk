//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the BackgroundNormalProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "alf:CT_BackgroundNormalProperties/alf:Normal")]
pub struct BackgroundNormalProperties {
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2021, qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundRemovedProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "alf:CT_BackgroundRemovedProperties/alf:Removed")]
pub struct BackgroundRemovedProperties {
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2021, qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundBlurProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "alf:CT_BackgroundBlurProperties/alf:Blur")]
pub struct BackgroundBlurProperties {
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2021, qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundCustomProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "alf:CT_BackgroundCustomProperties/alf:Custom")]
pub struct BackgroundCustomProperties {
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2021, qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LiveFeedProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "alf:CT_LiveFeedProperties/alf:liveFeedProps")]
pub struct LiveFeedProperties {
  /// Defines the LiveFeedBackgroundProperties Class.
  #[sdk(child(
    office2021,
    qname = "alf:CT_LiveFeedBackgroundProperties/alf:backgroundProps"
  ))]
  pub live_feed_background_properties: Option<std::boxed::Box<LiveFeedBackgroundProperties>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2021, qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "a:CT_OfficeArtExtensionList/alf:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
/// Defines the LiveFeedBackgroundProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "alf:CT_LiveFeedBackgroundProperties/alf:backgroundProps"
)]
pub struct LiveFeedBackgroundProperties {
  #[sdk(
        choice(
            child(variant = BackgroundNormalProperties, qname = "alf:Normal"),
            child(variant = BackgroundRemovedProperties, qname = "alf:Removed"),
            child(variant = BackgroundBlurProperties, qname = "alf:Blur"),
            child(variant = BackgroundCustomProperties, qname = "alf:Custom")
        )
    )]
  pub live_feed_background_properties_choice: Option<LiveFeedBackgroundPropertiesChoice>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2021, qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LiveFeedBackgroundPropertiesChoice {
  /// Defines the BackgroundNormalProperties Class.
  BackgroundNormalProperties(std::boxed::Box<BackgroundNormalProperties>),
  /// Defines the BackgroundRemovedProperties Class.
  BackgroundRemovedProperties(std::boxed::Box<BackgroundRemovedProperties>),
  /// Defines the BackgroundBlurProperties Class.
  BackgroundBlurProperties(std::boxed::Box<BackgroundBlurProperties>),
  /// Defines the BackgroundCustomProperties Class.
  BackgroundCustomProperties(std::boxed::Box<BackgroundCustomProperties>),
}
