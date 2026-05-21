//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WebExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "we:CT_OsfWebExtension/we:webextension")]
pub struct WebExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(std::boxed::Box<str>, std::boxed::Box<str>)>,
  /// id
  #[sdk(attr(office2013, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// frozen
  #[sdk(attr(office2013, qname = ":frozen"))]
  pub frozen: Option<crate::simple_type::BooleanValue>,
  /// Defines the WebExtensionStoreReference Class.
  #[sdk(child(office2013, qname = "we:CT_OsfWebExtensionReference/we:reference"))]
  pub web_extension_store_reference: std::boxed::Box<WebExtensionStoreReference>,
  /// Defines the WebExtensionReferenceList Class.
  #[sdk(child(
    office2013,
    qname = "we:CT_OsfWebExtensionReferenceList/we:alternateReferences"
  ))]
  pub web_extension_reference_list: Option<WebExtensionReferenceList>,
  /// Defines the WebExtensionPropertyBag Class.
  #[sdk(child(office2013, qname = "we:CT_OsfWebExtensionPropertyBag/we:properties"))]
  pub web_extension_property_bag: std::boxed::Box<WebExtensionPropertyBag>,
  /// Defines the WebExtensionBindingList Class.
  #[sdk(child(office2013, qname = "we:CT_OsfWebExtensionBindingList/we:bindings"))]
  pub web_extension_binding_list: std::boxed::Box<WebExtensionBindingList>,
  /// Defines the Snapshot Class.
  #[sdk(child(office2013, qname = "a:CT_Blip/we:snapshot"))]
  pub snapshot: Option<std::boxed::Box<Snapshot>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/we:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "we:CT_WebExtensionPartRef/we:webextensionref")]
pub struct WebExtensionReference {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// id
  #[sdk(attr(office2013, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the WebExtensionProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "we:CT_OsfWebExtensionProperty/we:property")]
pub struct WebExtensionProperty {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// name
  #[sdk(attr(office2013, qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// value
  #[sdk(attr(office2013, qname = ":value"))]
  pub value: crate::simple_type::StringValue,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "a:CT_OfficeArtExtensionList/we:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
/// Defines the WebExtensionBinding Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "we:CT_OsfWebExtensionBinding/we:binding")]
pub struct WebExtensionBinding {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// id
  #[sdk(attr(office2013, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// type
  #[sdk(attr(office2013, qname = ":type"))]
  pub r#type: crate::simple_type::StringValue,
  /// appref
  #[sdk(attr(office2013, qname = ":appref"))]
  pub app_reference: crate::simple_type::StringValue,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/we:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionStoreReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "we:CT_OsfWebExtensionReference/we:reference")]
pub struct WebExtensionStoreReference {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// id
  #[sdk(attr(office2013, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// version
  #[sdk(attr(office2013, qname = ":version"))]
  pub version: crate::simple_type::StringValue,
  /// store
  #[sdk(attr(office2013, qname = ":store"))]
  pub store: Option<crate::simple_type::StringValue>,
  /// storeType
  #[sdk(attr(office2013, qname = ":storeType"))]
  pub store_type: Option<crate::simple_type::StringValue>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/we:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionReferenceList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2013,
  qname = "we:CT_OsfWebExtensionReferenceList/we:alternateReferences"
)]
pub struct WebExtensionReferenceList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the WebExtensionStoreReference Class.
  #[sdk(child(office2013, qname = "we:CT_OsfWebExtensionReference/we:reference"))]
  pub web_extension_store_reference: Vec<WebExtensionStoreReference>,
}
/// Defines the WebExtensionPropertyBag Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "we:CT_OsfWebExtensionPropertyBag/we:properties")]
pub struct WebExtensionPropertyBag {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the WebExtensionProperty Class.
  #[sdk(child(office2013, qname = "we:CT_OsfWebExtensionProperty/we:property"))]
  pub web_extension_property: Vec<WebExtensionProperty>,
}
/// Defines the WebExtensionBindingList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "we:CT_OsfWebExtensionBindingList/we:bindings")]
pub struct WebExtensionBindingList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the WebExtensionBinding Class.
  #[sdk(child(office2013, qname = "we:CT_OsfWebExtensionBinding/we:binding"))]
  pub web_extension_binding: Vec<WebExtensionBinding>,
}
/// Defines the Snapshot Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "a:CT_Blip/we:snapshot")]
pub struct Snapshot {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Embedded Picture Reference
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// Compression state for blips.
  #[sdk(attr(qname = ":cstate"))]
  #[sdk(string_format(kind = "token"))]
  pub compression_state: Option<crate::schemas::a::BlipCompressionValues>,
  #[sdk(choice(
    qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel",
    qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling",
    qname = "a:CT_AlphaFloorEffect/a:alphaFloor",
    qname = "a:CT_AlphaInverseEffect/a:alphaInv",
    qname = "a:CT_AlphaModulateEffect/a:alphaMod",
    qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix",
    qname = "a:CT_AlphaReplaceEffect/a:alphaRepl",
    qname = "a:CT_BiLevelEffect/a:biLevel",
    qname = "a:CT_BlurEffect/a:blur",
    qname = "a:CT_ColorChangeEffect/a:clrChange",
    qname = "a:CT_ColorReplaceEffect/a:clrRepl",
    qname = "a:CT_DuotoneEffect/a:duotone",
    qname = "a:CT_FillOverlayEffect/a:fillOverlay",
    qname = "a:CT_GrayscaleEffect/a:grayscl",
    qname = "a:CT_HSLEffect/a:hsl",
    qname = "a:CT_LuminanceEffect/a:lum",
    qname = "a:CT_TintEffect/a:tint"
  ))]
  pub snapshot_choice: Vec<SnapshotChoice>,
  /// Future extensions..
  #[sdk(child(qname = "a:CT_BlipExtensionList/a:extLst"))]
  pub blip_extension_list: Option<crate::schemas::a::BlipExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SnapshotChoice {
  AlphaBiLevel(std::boxed::Box<crate::schemas::a::AlphaBiLevel>),
  /// Alpha Ceiling Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling"))]
  AlphaCeiling,
  /// Alpha Floor Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaFloorEffect/a:alphaFloor"))]
  AlphaFloor,
  AlphaInverse(std::boxed::Box<crate::schemas::a::AlphaInverse>),
  AlphaModulationEffect(std::boxed::Box<crate::schemas::a::AlphaModulationEffect>),
  AlphaModulationFixed(std::boxed::Box<crate::schemas::a::AlphaModulationFixed>),
  AlphaReplace(std::boxed::Box<crate::schemas::a::AlphaReplace>),
  BiLevel(std::boxed::Box<crate::schemas::a::BiLevel>),
  Blur(std::boxed::Box<crate::schemas::a::Blur>),
  ColorChange(std::boxed::Box<crate::schemas::a::ColorChange>),
  ColorReplacement(std::boxed::Box<crate::schemas::a::ColorReplacement>),
  Duotone(std::boxed::Box<crate::schemas::a::Duotone>),
  FillOverlay(std::boxed::Box<crate::schemas::a::FillOverlay>),
  /// Gray Scale Effect.
  #[sdk(empty_child(qname = "a:CT_GrayscaleEffect/a:grayscl"))]
  Grayscale,
  Hsl(std::boxed::Box<crate::schemas::a::Hsl>),
  LuminanceEffect(std::boxed::Box<crate::schemas::a::LuminanceEffect>),
  TintEffect(std::boxed::Box<crate::schemas::a::TintEffect>),
}
