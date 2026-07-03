//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CustomXsn Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ntns:customXsn")]
pub struct CustomXsn {
  /// Defines the XsnLocation Class.
  #[sdk(text_child(qname = "ntns:xsnLocation"))]
  pub xsn_location: XsnLocation,
  /// Defines the CachedView Class.
  #[sdk(text_child(qname = "ntns:cached"))]
  pub cached_view: CachedView,
  /// Defines the OpenByDefault Class.
  #[sdk(text_child(qname = "ntns:openByDefault"))]
  pub open_by_default: OpenByDefault,
  /// Defines the Scope Class.
  #[sdk(text_child(qname = "ntns:xsnScope"))]
  pub scope: Scope,
}
/// Defines the XsnLocation Class.
pub type XsnLocation = crate::simple_type::StringValue;
/// Defines the CachedView Class.
pub type CachedView = crate::simple_type::StringValue;
/// Defines the OpenByDefault Class.
pub type OpenByDefault = crate::simple_type::StringValue;
/// Defines the Scope Class.
pub type Scope = crate::simple_type::StringValue;
