//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CustomXsn Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ntns:CT_CustomXsn/ntns:customXsn")]
pub struct CustomXsn {
  /// Defines the XsnLocation Class.
  #[sdk(text_child(qname = "xsd:string/ntns:xsnLocation"))]
  pub xsn_location: crate::simple_type::StringValue,
  /// Defines the CachedView Class.
  #[sdk(text_child(qname = "xsd:string/ntns:cached"))]
  pub cached_view: crate::simple_type::StringValue,
  /// Defines the OpenByDefault Class.
  #[sdk(text_child(qname = "xsd:string/ntns:openByDefault"))]
  pub open_by_default: crate::simple_type::StringValue,
  /// Defines the Scope Class.
  #[sdk(text_child(qname = "xsd:string/ntns:xsnScope"))]
  pub scope: crate::simple_type::StringValue,
}
/// Defines the XsnLocation Class.
pub type XsnLocation = crate::simple_type::StringValue;
/// Defines the CachedView Class.
pub type CachedView = crate::simple_type::StringValue;
/// Defines the OpenByDefault Class.
pub type OpenByDefault = crate::simple_type::StringValue;
/// Defines the Scope Class.
pub type Scope = crate::simple_type::StringValue;
