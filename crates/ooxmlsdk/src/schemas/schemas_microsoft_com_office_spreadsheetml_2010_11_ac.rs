//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the AbsolutePath Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is x15ac:absPath.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x15ac:CT_AbsolutePath/x15ac:absPath")]
pub struct AbsolutePath {
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
  /// url
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :url
  #[sdk(attr(qname = ":url"))]
  pub url: crate::simple_type::StringValue,
}
