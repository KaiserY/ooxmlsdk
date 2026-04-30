//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the OEmbed Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "woe:CT_OEmbed/woe:oembed")]
pub struct OEmbed {
  /// oEmbedUrl
  #[sdk(attr(microsoft365, qname = ":oEmbedUrl"))]
  pub o_embed_url: crate::simple_type::StringValue,
  /// mediaType
  #[sdk(attr(microsoft365, qname = ":mediaType"))]
  pub media_type: crate::simple_type::StringValue,
  /// picLocksAutoForOEmbed
  #[sdk(attr(microsoft365, qname = ":picLocksAutoForOEmbed"))]
  pub pic_locks_auto_for_o_embed: Option<crate::simple_type::BooleanValue>,
}
