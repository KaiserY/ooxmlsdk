//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the WebVideoProperty Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is wp15:webVideoPr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp15:CT_WebVideoPr/wp15:webVideoPr")]
pub struct WebVideoProperty {
  /// embeddedHtml
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :embeddedHtml
  #[sdk(attr(qname = ":embeddedHtml"))]
  pub embedded_html: Option<crate::simple_type::StringValue>,
  /// h
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :h
  #[sdk(attr(qname = ":h"))]
  pub height: Option<crate::simple_type::UInt32Value>,
  /// w
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :w
  #[sdk(attr(qname = ":w"))]
  pub width: Option<crate::simple_type::UInt32Value>,
}
