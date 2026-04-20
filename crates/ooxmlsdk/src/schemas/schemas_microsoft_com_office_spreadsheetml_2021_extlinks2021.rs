//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ExternalBookAlternateUrls Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxl21:alternateUrls.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxl21:CT_ExternalBookAlternateUrls/xxl21:alternateUrls")]
pub struct ExternalBookAlternateUrls {
  /// driveId
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :driveId
  #[sdk(attr(qname = ":driveId"))]
  pub drive_id: Option<crate::simple_type::StringValue>,
  /// itemId
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :itemId
  #[sdk(attr(qname = ":itemId"))]
  pub item_id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "xxl21:CT_AlternateUrl/xxl21:absoluteUrl"))]
  pub absolute_url_alternate_url: Option<AbsoluteUrlAlternateUrl>,
  /// _
  #[sdk(child(qname = "xxl21:CT_AlternateUrl/xxl21:relativeUrl"))]
  pub relative_url_alternate_url: Option<RelativeUrlAlternateUrl>,
}
/// Defines the AbsoluteUrlAlternateUrl Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxl21:absoluteUrl.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxl21:CT_AlternateUrl/xxl21:absoluteUrl")]
pub struct AbsoluteUrlAlternateUrl {
  /// id
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the RelativeUrlAlternateUrl Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxl21:relativeUrl.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxl21:CT_AlternateUrl/xxl21:relativeUrl")]
pub struct RelativeUrlAlternateUrl {
  /// id
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
/// Defines the OpenXmlAlternateUrlElement Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxl21:CT_AlternateUrl/")]
pub struct OpenXmlAlternateUrlElement {
  /// id
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
