//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WebVideoProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp15:webVideoPr")]
pub struct WebVideoProperty {
  /// embeddedHtml
  #[sdk(attr(qname = ":embeddedHtml"))]
  pub embedded_html: Option<crate::simple_type::StringValue>,
  /// h
  #[sdk(attr(qname = ":h"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// w
  #[sdk(attr(qname = ":w"))]
  pub width: Option<crate::simple_type::UInt32Value>,
}
