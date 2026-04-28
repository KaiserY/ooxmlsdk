//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Version Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlwcv:version.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlwcv:CT_Version/xlwcv:version")]
pub struct Version {
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
  /// warnBelowVersion
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :warnBelowVersion
  #[sdk(attr(qname = ":warnBelowVersion"))]
  pub warn_below_version: Option<crate::simple_type::UInt32Value>,
  /// setVersion
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :setVersion
  #[sdk(attr(qname = ":setVersion"))]
  pub set_version: Option<crate::simple_type::UInt32Value>,
}
