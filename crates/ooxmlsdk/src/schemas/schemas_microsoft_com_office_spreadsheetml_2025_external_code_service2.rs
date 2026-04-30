//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ExternalCodeServiceImageAsInput Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  microsoft365,
  qname = "xlecs2:CT_ExternalCodeServiceImageAsInput/xlecs2:externalCodeServiceImageAsInput"
)]
pub struct ExternalCodeServiceImageAsInput {
  /// maxWidth
  #[sdk(attr(microsoft365, qname = ":maxWidth"))]
  pub max_width: Option<crate::simple_type::UInt32Value>,
  /// maxHeight
  #[sdk(attr(microsoft365, qname = ":maxHeight"))]
  pub max_height: Option<crate::simple_type::UInt32Value>,
}
