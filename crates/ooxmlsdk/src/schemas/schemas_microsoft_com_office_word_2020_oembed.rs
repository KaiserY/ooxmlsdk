//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the OEmbed Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is woe:oembed.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "woe:CT_OEmbed/woe:oembed")]
pub struct OEmbed {
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
  /// oEmbedUrl
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :oEmbedUrl
  #[sdk(attr(qname = ":oEmbedUrl"))]
  pub o_embed_url: crate::simple_type::StringValue,
  /// mediaType
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :mediaType
  #[sdk(attr(qname = ":mediaType"))]
  pub media_type: crate::simple_type::StringValue,
  /// picLocksAutoForOEmbed
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :picLocksAutoForOEmbed
  #[sdk(attr(qname = ":picLocksAutoForOEmbed"))]
  pub pic_locks_auto_for_o_embed: Option<crate::simple_type::BooleanValue>,
}
