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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "wp15:CT_WebVideoPr/wp15:webVideoPr")]
pub struct WebVideoProperty {
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
