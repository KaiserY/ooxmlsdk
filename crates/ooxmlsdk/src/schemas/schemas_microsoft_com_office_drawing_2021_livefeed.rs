//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the BackgroundNormalProperties Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is alf:Normal.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "alf:CT_BackgroundNormalProperties/alf:Normal")]
pub struct BackgroundNormalProperties {
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundRemovedProperties Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is alf:Removed.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "alf:CT_BackgroundRemovedProperties/alf:Removed")]
pub struct BackgroundRemovedProperties {
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundBlurProperties Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is alf:Blur.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "alf:CT_BackgroundBlurProperties/alf:Blur")]
pub struct BackgroundBlurProperties {
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the BackgroundCustomProperties Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is alf:Custom.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "alf:CT_BackgroundCustomProperties/alf:Custom")]
pub struct BackgroundCustomProperties {
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the LiveFeedProperties Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is alf:liveFeedProps.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "alf:CT_LiveFeedProperties/alf:liveFeedProps")]
pub struct LiveFeedProperties {
  /// _
  #[sdk(child(qname = "alf:CT_LiveFeedBackgroundProperties/alf:backgroundProps"))]
  pub live_feed_background_properties: Option<std::boxed::Box<LiveFeedBackgroundProperties>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is alf:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/alf:extLst")]
pub struct OfficeArtExtensionList {
  ///Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the LiveFeedBackgroundProperties Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is alf:backgroundProps.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "alf:CT_LiveFeedBackgroundProperties/alf:backgroundProps")]
pub struct LiveFeedBackgroundProperties {
  #[sdk(choice(
    qname = "alf:CT_BackgroundNormalProperties/alf:Normal",
    qname = "alf:CT_BackgroundRemovedProperties/alf:Removed",
    qname = "alf:CT_BackgroundBlurProperties/alf:Blur",
    qname = "alf:CT_BackgroundCustomProperties/alf:Custom"
  ))]
  pub live_feed_background_properties_choice: Option<LiveFeedBackgroundPropertiesChoice>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/alf:extLst"))]
  pub alf_ext_lst: Option<OfficeArtExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LiveFeedBackgroundPropertiesChoice {
  #[sdk(child(qname = "alf:CT_BackgroundNormalProperties/alf:Normal"))]
  AlfNormal(std::boxed::Box<BackgroundNormalProperties>),
  #[sdk(child(qname = "alf:CT_BackgroundRemovedProperties/alf:Removed"))]
  AlfRemoved(std::boxed::Box<BackgroundRemovedProperties>),
  #[sdk(child(qname = "alf:CT_BackgroundBlurProperties/alf:Blur"))]
  AlfBlur(std::boxed::Box<BackgroundBlurProperties>),
  #[sdk(child(qname = "alf:CT_BackgroundCustomProperties/alf:Custom"))]
  AlfCustom(std::boxed::Box<BackgroundCustomProperties>),
}
