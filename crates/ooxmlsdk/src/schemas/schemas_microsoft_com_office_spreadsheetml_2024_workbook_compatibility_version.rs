//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Version Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(microsoft365, qname = "xlwcv:CT_Version/xlwcv:version")]
pub struct Version {
  /// warnBelowVersion
  #[sdk(attr(microsoft365, qname = ":warnBelowVersion"))]
  pub warn_below_version: Option<crate::simple_type::UInt32Value>,
  /// setVersion
  #[sdk(attr(microsoft365, qname = ":setVersion"))]
  pub set_version: Option<crate::simple_type::UInt32Value>,
}
