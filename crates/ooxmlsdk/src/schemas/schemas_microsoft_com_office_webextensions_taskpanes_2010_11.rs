//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Taskpanes Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "wetp:CT_OsfTaskpanes/wetp:taskpanes")]
pub struct Taskpanes {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the WebExtensionTaskpane Class.
  #[sdk(child(office2013, qname = "wetp:CT_OsfTaskpane/wetp:taskpane"))]
  pub wetp_taskpane: Vec<WebExtensionTaskpane>,
}
/// Defines the WebExtensionPartReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "we:CT_WebExtensionPartRef/wetp:webextensionref")]
pub struct WebExtensionPartReference {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// id
  #[sdk(attr(office2013, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "a:CT_OfficeArtExtensionList/wetp:extLst")]
pub struct OfficeArtExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "a:CT_OfficeArtExtension/a:ext"))]
  pub a_ext: Vec<crate::schemas::a::Extension>,
}
/// Defines the WebExtensionTaskpane Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "wetp:CT_OsfTaskpane/wetp:taskpane")]
pub struct WebExtensionTaskpane {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// dockstate
  #[sdk(attr(office2013, qname = ":dockstate"))]
  pub dock_state: crate::simple_type::StringValue,
  /// visibility
  #[sdk(attr(office2013, qname = ":visibility"))]
  pub visibility: crate::simple_type::BooleanValue,
  /// width
  #[sdk(attr(office2013, qname = ":width"))]
  pub width: crate::simple_type::DoubleValue,
  /// row
  #[sdk(attr(office2013, qname = ":row"))]
  pub row: crate::simple_type::UInt32Value,
  /// locked
  #[sdk(attr(office2013, qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// Defines the WebExtensionPartReference Class.
  #[sdk(child(office2013, qname = "we:CT_WebExtensionPartRef/wetp:webextensionref"))]
  pub web_extension_part_reference: std::boxed::Box<WebExtensionPartReference>,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(office2013, qname = "a:CT_OfficeArtExtensionList/wetp:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
