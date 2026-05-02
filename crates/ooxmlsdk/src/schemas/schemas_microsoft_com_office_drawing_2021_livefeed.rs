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
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub a_ext: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the LiveFeedBackgroundProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "alf:CT_LiveFeedBackgroundProperties/alf:backgroundProps"
)]
pub struct LiveFeedBackgroundProperties {
  #[sdk(choice(
    qname = "alf:CT_BackgroundNormalProperties/alf:Normal",
    qname = "alf:CT_BackgroundRemovedProperties/alf:Removed",
    qname = "alf:CT_BackgroundBlurProperties/alf:Blur",
    qname = "alf:CT_BackgroundCustomProperties/alf:Custom"
  ))]
  pub live_feed_background_properties_choice: Option<LiveFeedBackgroundPropertiesChoice>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2021, qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub alf_ext_lst: Option<OfficeArtExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LiveFeedBackgroundPropertiesChoice {
  #[sdk(child(office2021, qname = "alf:CT_BackgroundNormalProperties/alf:Normal"))]
  AlfNormal(std::boxed::Box<BackgroundNormalProperties>),
  #[sdk(child(office2021, qname = "alf:CT_BackgroundRemovedProperties/alf:Removed"))]
  AlfRemoved(std::boxed::Box<BackgroundRemovedProperties>),
  #[sdk(child(office2021, qname = "alf:CT_BackgroundBlurProperties/alf:Blur"))]
  AlfBlur(std::boxed::Box<BackgroundBlurProperties>),
  #[sdk(child(office2021, qname = "alf:CT_BackgroundCustomProperties/alf:Custom"))]
  AlfCustom(std::boxed::Box<BackgroundCustomProperties>),
}
