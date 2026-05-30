//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ExternalCodeService Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlecs:externalCodeService")]
pub struct ExternalCodeService {
  /// autoShow
  #[sdk(attr(qname = ":autoShow"))]
  pub auto_show: Option<crate::simple_type::UInt32Value>,
  /// timeout
  #[sdk(attr(qname = ":timeout"))]
  pub timeout: Option<crate::simple_type::UInt32Value>,
}
