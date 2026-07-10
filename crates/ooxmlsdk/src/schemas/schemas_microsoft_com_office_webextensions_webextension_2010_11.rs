//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WebExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "we:webextension")]
pub struct WebExtension {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// frozen
  #[sdk(attr(qname = ":frozen"))]
  pub frozen: Option<crate::simple_type::BooleanValue>,
  /// Defines the WebExtensionStoreReference Class.
  #[sdk(child(qname = "we:reference"))]
  pub web_extension_store_reference: std::boxed::Box<WebExtensionStoreReference>,
  /// Defines the WebExtensionReferenceList Class.
  #[sdk(child(qname = "we:alternateReferences"))]
  pub web_extension_reference_list: Option<WebExtensionReferenceList>,
  /// Defines the WebExtensionPropertyBag Class.
  #[sdk(child(qname = "we:properties"))]
  pub web_extension_property_bag: WebExtensionPropertyBag,
  /// Defines the WebExtensionBindingList Class.
  #[sdk(child(qname = "we:bindings"))]
  pub web_extension_binding_list: WebExtensionBindingList,
  /// Defines the Snapshot Class.
  #[sdk(child(qname = "we:snapshot"))]
  pub snapshot: Option<std::boxed::Box<Snapshot>>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "we:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:webextensionref")]
pub struct WebExtensionReference {
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the WebExtensionProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:property")]
pub struct WebExtensionProperty {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// value
  #[sdk(attr(qname = ":value"))]
  pub value: crate::simple_type::StringValue,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
/// Defines the WebExtensionBinding Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:binding")]
pub struct WebExtensionBinding {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// type
  #[sdk(attr(qname = ":type"))]
  pub r#type: crate::simple_type::StringValue,
  /// appref
  #[sdk(attr(qname = ":appref"))]
  pub app_reference: crate::simple_type::StringValue,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "we:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionStoreReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:reference")]
pub struct WebExtensionStoreReference {
  /// id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// version
  #[sdk(attr(qname = ":version"))]
  pub version: crate::simple_type::StringValue,
  /// store
  #[sdk(attr(qname = ":store"))]
  pub store: Option<crate::simple_type::StringValue>,
  /// storeType
  #[sdk(attr(qname = ":storeType"))]
  pub store_type: Option<crate::simple_type::StringValue>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "we:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionReferenceList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:alternateReferences")]
pub struct WebExtensionReferenceList {
  /// Defines the WebExtensionStoreReference Class.
  #[sdk(child(qname = "we:reference"))]
  pub web_extension_store_reference: Vec<WebExtensionStoreReference>,
}
/// Defines the WebExtensionPropertyBag Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:properties")]
pub struct WebExtensionPropertyBag {
  /// Defines the WebExtensionProperty Class.
  #[sdk(child(qname = "we:property"))]
  pub web_extension_property: Vec<WebExtensionProperty>,
}
/// Defines the WebExtensionBindingList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:bindings")]
pub struct WebExtensionBindingList {
  /// Defines the WebExtensionBinding Class.
  #[sdk(child(qname = "we:binding"))]
  pub web_extension_binding: Vec<WebExtensionBinding>,
}
/// Defines the Snapshot Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:snapshot")]
pub struct Snapshot {
  pub xmlns: Vec<crate::common::XmlNamespace>,
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
  #[sdk(
        choice(
            child(variant = AlphaBiLevel, qname = "a:alphaBiLevel"),
            empty_child(variant = AlphaCeiling, qname = "a:alphaCeiling"),
            empty_child(variant = AlphaFloor, qname = "a:alphaFloor"),
            child(variant = AlphaInverse, boxed, qname = "a:alphaInv"),
            child(variant = AlphaModulationEffect, boxed, qname = "a:alphaMod"),
            child(variant = AlphaModulationFixed, qname = "a:alphaModFix"),
            child(variant = AlphaReplace, qname = "a:alphaRepl"),
            child(variant = BiLevel, qname = "a:biLevel"),
            child(variant = Blur, qname = "a:blur"),
            child(variant = ColorChange, boxed, qname = "a:clrChange"),
            child(variant = ColorReplacement, boxed, qname = "a:clrRepl"),
            child(variant = Duotone, qname = "a:duotone"),
            child(variant = FillOverlay, boxed, qname = "a:fillOverlay"),
            empty_child(variant = Grayscale, qname = "a:grayscl"),
            child(variant = Hsl, qname = "a:hsl"),
            child(variant = LuminanceEffect, qname = "a:lum"),
            child(variant = TintEffect, qname = "a:tint")
        )
    )]
  pub snapshot_choice: Vec<SnapshotChoice>,
  /// Future extensions..
  #[sdk(child(qname = "a:extLst"))]
  pub blip_extension_list: Option<crate::schemas::a::BlipExtensionList>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum SnapshotChoice {
  AlphaBiLevel(crate::schemas::a::AlphaBiLevel),
  /// Alpha Ceiling Effect.
  AlphaCeiling,
  /// Alpha Floor Effect.
  AlphaFloor,
  AlphaInverse(std::boxed::Box<crate::schemas::a::AlphaInverse>),
  AlphaModulationEffect(std::boxed::Box<crate::schemas::a::AlphaModulationEffect>),
  AlphaModulationFixed(crate::schemas::a::AlphaModulationFixed),
  AlphaReplace(crate::schemas::a::AlphaReplace),
  BiLevel(crate::schemas::a::BiLevel),
  Blur(crate::schemas::a::Blur),
  ColorChange(std::boxed::Box<crate::schemas::a::ColorChange>),
  ColorReplacement(std::boxed::Box<crate::schemas::a::ColorReplacement>),
  Duotone(crate::schemas::a::Duotone),
  FillOverlay(std::boxed::Box<crate::schemas::a::FillOverlay>),
  /// Gray Scale Effect.
  Grayscale,
  Hsl(crate::schemas::a::Hsl),
  LuminanceEffect(crate::schemas::a::LuminanceEffect),
  TintEffect(crate::schemas::a::TintEffect),
}
