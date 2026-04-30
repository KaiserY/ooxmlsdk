//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WebVideoProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2013, qname = "wp15:CT_WebVideoPr/wp15:webVideoPr")]
pub struct WebVideoProperty {
  /// embeddedHtml
  #[sdk(attr(office2013, qname = ":embeddedHtml"))]
  pub embedded_html: Option<crate::simple_type::StringValue>,
  /// h
  #[sdk(attr(office2013, qname = ":h"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// w
  #[sdk(attr(office2013, qname = ":w"))]
  pub width: Option<crate::simple_type::UInt32Value>,
}
