//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WebExtension Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is we:webextension.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:CT_OsfWebExtension/we:webextension")]
pub struct WebExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// frozen
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :frozen
  #[sdk(attr(qname = ":frozen"))]
  pub frozen: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "we:CT_OsfWebExtensionReference/we:reference"))]
  pub web_extension_store_reference: std::boxed::Box<WebExtensionStoreReference>,
  /// _
  #[sdk(child(qname = "we:CT_OsfWebExtensionReferenceList/we:alternateReferences"))]
  pub web_extension_reference_list: Option<WebExtensionReferenceList>,
  /// _
  #[sdk(child(qname = "we:CT_OsfWebExtensionPropertyBag/we:properties"))]
  pub web_extension_property_bag: std::boxed::Box<WebExtensionPropertyBag>,
  /// _
  #[sdk(child(qname = "we:CT_OsfWebExtensionBindingList/we:bindings"))]
  pub web_extension_binding_list: std::boxed::Box<WebExtensionBindingList>,
  /// _
  #[sdk(child(qname = "a:CT_Blip/we:snapshot"))]
  pub snapshot: Option<std::boxed::Box<Snapshot>>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/we:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionReference Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is we:webextensionref.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:CT_WebExtensionPartRef/we:webextensionref")]
pub struct WebExtensionReference {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the WebExtensionProperty Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is we:property.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:CT_OsfWebExtensionProperty/we:property")]
pub struct WebExtensionProperty {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// value
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :value
  #[sdk(attr(qname = ":value"))]
  pub value: crate::simple_type::StringValue,
}
/// Defines the OfficeArtExtensionList Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is we:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_OfficeArtExtensionList/we:extLst")]
pub struct OfficeArtExtensionList {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extension>,
}
/// Defines the WebExtensionBinding Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is we:binding.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:CT_OsfWebExtensionBinding/we:binding")]
pub struct WebExtensionBinding {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// type
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :type
  #[sdk(attr(qname = ":type"))]
  pub r#type: crate::simple_type::StringValue,
  /// appref
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :appref
  #[sdk(attr(qname = ":appref"))]
  pub app_reference: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/we:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionStoreReference Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is we:reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:CT_OsfWebExtensionReference/we:reference")]
pub struct WebExtensionStoreReference {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// version
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :version
  #[sdk(attr(qname = ":version"))]
  pub version: crate::simple_type::StringValue,
  /// store
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :store
  #[sdk(attr(qname = ":store"))]
  pub store: Option<crate::simple_type::StringValue>,
  /// storeType
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :storeType
  #[sdk(attr(qname = ":storeType"))]
  pub store_type: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "a:CT_OfficeArtExtensionList/we:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
/// Defines the WebExtensionReferenceList Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is we:alternateReferences.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:CT_OsfWebExtensionReferenceList/we:alternateReferences")]
pub struct WebExtensionReferenceList {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "we:CT_OsfWebExtensionReference/we:reference"))]
  pub we_reference: Vec<WebExtensionStoreReference>,
}
/// Defines the WebExtensionPropertyBag Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is we:properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:CT_OsfWebExtensionPropertyBag/we:properties")]
pub struct WebExtensionPropertyBag {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "we:CT_OsfWebExtensionProperty/we:property"))]
  pub we_property: Vec<WebExtensionProperty>,
}
/// Defines the WebExtensionBindingList Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is we:bindings.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "we:CT_OsfWebExtensionBindingList/we:bindings")]
pub struct WebExtensionBindingList {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "we:CT_OsfWebExtensionBinding/we:binding"))]
  pub we_binding: Vec<WebExtensionBinding>,
}
/// Defines the Snapshot Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is we:snapshot.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Blip/we:snapshot")]
pub struct Snapshot {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// Embedded Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:embed
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: r:link
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// Compression state for blips.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :cstate
  #[sdk(attr(qname = ":cstate"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub compression_state:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipCompressionValues>,
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
  /// _
  #[sdk(child(qname = "a:CT_BlipExtensionList/a:extLst"))]
  pub a_ext_lst:
    Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BlipExtensionList>,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum SnapshotChoice {
  #[sdk(child(qname = "a:CT_AlphaBiLevelEffect/a:alphaBiLevel"))]
  AAlphaBiLevel(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaBiLevel>,
  ),
  /// Alpha Ceiling Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaCeilingEffect/a:alphaCeiling"))]
  AAlphaCeiling,
  /// Alpha Floor Effect.
  #[sdk(empty_child(qname = "a:CT_AlphaFloorEffect/a:alphaFloor"))]
  AAlphaFloor,
  #[sdk(child(qname = "a:CT_AlphaInverseEffect/a:alphaInv"))]
  AAlphaInv(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaInverse>,
  ),
  #[sdk(child(qname = "a:CT_AlphaModulateEffect/a:alphaMod"))]
  AAlphaMod(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulationEffect,
    >,
  ),
  #[sdk(child(qname = "a:CT_AlphaModulateFixedEffect/a:alphaModFix"))]
  AAlphaModFix(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaModulationFixed,
    >,
  ),
  #[sdk(child(qname = "a:CT_AlphaReplaceEffect/a:alphaRepl"))]
  AAlphaRepl(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AlphaReplace>,
  ),
  #[sdk(child(qname = "a:CT_BiLevelEffect/a:biLevel"))]
  ABiLevel(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::BiLevel>,
  ),
  #[sdk(child(qname = "a:CT_BlurEffect/a:blur"))]
  ABlur(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Blur>),
  #[sdk(child(qname = "a:CT_ColorChangeEffect/a:clrChange"))]
  AClrChange(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorChange>,
  ),
  #[sdk(child(qname = "a:CT_ColorReplaceEffect/a:clrRepl"))]
  AClrRepl(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::ColorReplacement,
    >,
  ),
  #[sdk(child(qname = "a:CT_DuotoneEffect/a:duotone"))]
  ADuotone(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Duotone>,
  ),
  #[sdk(child(qname = "a:CT_FillOverlayEffect/a:fillOverlay"))]
  AFillOverlay(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::FillOverlay>,
  ),
  /// Gray Scale Effect.
  #[sdk(empty_child(qname = "a:CT_GrayscaleEffect/a:grayscl"))]
  AGrayscl,
  #[sdk(child(qname = "a:CT_HSLEffect/a:hsl"))]
  AHsl(std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Hsl>),
  #[sdk(child(qname = "a:CT_LuminanceEffect/a:lum"))]
  ALum(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::LuminanceEffect,
    >,
  ),
  #[sdk(child(qname = "a:CT_TintEffect/a:tint"))]
  ATint(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::TintEffect>,
  ),
}
