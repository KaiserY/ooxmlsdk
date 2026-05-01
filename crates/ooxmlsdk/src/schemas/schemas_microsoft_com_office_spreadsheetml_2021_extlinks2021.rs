//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ExternalBookAlternateUrls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xxl21:CT_ExternalBookAlternateUrls/xxl21:alternateUrls"
)]
pub struct ExternalBookAlternateUrls {
  /// driveId
  #[sdk(attr(microsoft365, qname = ":driveId"))]
  pub drive_id: Option<crate::simple_type::StringValue>,
  /// itemId
  #[sdk(attr(microsoft365, qname = ":itemId"))]
  pub item_id: Option<crate::simple_type::StringValue>,
  /// Defines the AbsoluteUrlAlternateUrl Class.
  #[sdk(child(microsoft365, qname = "xxl21:CT_AlternateUrl/xxl21:absoluteUrl"))]
  pub absolute_url_alternate_url: Option<AbsoluteUrlAlternateUrl>,
  /// Defines the RelativeUrlAlternateUrl Class.
  #[sdk(child(microsoft365, qname = "xxl21:CT_AlternateUrl/xxl21:relativeUrl"))]
  pub relative_url_alternate_url: Option<RelativeUrlAlternateUrl>,
}
/// Defines the AbsoluteUrlAlternateUrl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xxl21:CT_AlternateUrl/xxl21:absoluteUrl")]
pub struct AbsoluteUrlAlternateUrl {
  /// id
  #[sdk(attr(microsoft365, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the RelativeUrlAlternateUrl Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xxl21:CT_AlternateUrl/xxl21:relativeUrl")]
pub struct RelativeUrlAlternateUrl {
  /// id
  #[sdk(attr(microsoft365, qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
