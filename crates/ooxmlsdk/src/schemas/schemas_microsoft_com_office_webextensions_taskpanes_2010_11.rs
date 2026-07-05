//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Taskpanes Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "wetp:taskpanes")]
pub struct Taskpanes {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Defines the WebExtensionTaskpane Class.
  #[sdk(child(qname = "wetp:taskpane"))]
  pub web_extension_taskpane: Vec<WebExtensionTaskpane>,
}
/// Defines the WebExtensionPartReference Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wetp:webextensionref")]
pub struct WebExtensionPartReference {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the OfficeArtExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wetp:extLst")]
pub struct OfficeArtExtensionList {
  /// Extension.
  #[sdk(child(qname = "a:ext"))]
  pub extension: Vec<crate::schemas::a::Extension>,
}
/// Defines the WebExtensionTaskpane Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wetp:taskpane")]
pub struct WebExtensionTaskpane {
  /// dockstate
  #[sdk(attr(qname = ":dockstate"))]
  pub dock_state: crate::simple_type::StringValue,
  /// visibility
  #[sdk(attr(qname = ":visibility"))]
  pub visibility: crate::simple_type::BooleanValue,
  /// width
  #[sdk(attr(qname = ":width"))]
  pub width: crate::simple_type::DoubleValue,
  /// row
  #[sdk(attr(qname = ":row"))]
  pub row: crate::simple_type::UInt32Value,
  /// locked
  #[sdk(attr(qname = ":locked"))]
  pub locked: Option<crate::simple_type::BooleanValue>,
  /// Defines the WebExtensionPartReference Class.
  #[sdk(child(qname = "wetp:webextensionref"))]
  pub web_extension_part_reference: WebExtensionPartReference,
  /// Defines the OfficeArtExtensionList Class.
  #[sdk(child(qname = "wetp:extLst"))]
  pub office_art_extension_list: Option<OfficeArtExtensionList>,
}
