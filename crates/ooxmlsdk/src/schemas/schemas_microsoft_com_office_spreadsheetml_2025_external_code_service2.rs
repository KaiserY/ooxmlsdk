//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ExternalCodeServiceImageAsInput Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlecs2:externalCodeServiceImageAsInput.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlecs2:CT_ExternalCodeServiceImageAsInput/xlecs2:externalCodeServiceImageAsInput")]
pub struct ExternalCodeServiceImageAsInput {
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
  /// maxWidth
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :maxWidth
  #[sdk(attr(qname = ":maxWidth"))]
  pub max_width: Option<crate::simple_type::UInt32Value>,
  /// maxHeight
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :maxHeight
  #[sdk(attr(qname = ":maxHeight"))]
  pub max_height: Option<crate::simple_type::UInt32Value>,
}
