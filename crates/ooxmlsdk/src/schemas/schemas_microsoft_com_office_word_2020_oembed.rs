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
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "woe:CT_OEmbed/woe:oembed")]
pub struct OEmbed {
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
