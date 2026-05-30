//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Version Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlwcv:version")]
pub struct Version {
  /// warnBelowVersion
  #[sdk(attr(qname = ":warnBelowVersion"))]
  pub warn_below_version: Option<crate::simple_type::UInt32Value>,
  /// setVersion
  #[sdk(attr(qname = ":setVersion"))]
  pub set_version: Option<crate::simple_type::UInt32Value>,
}
