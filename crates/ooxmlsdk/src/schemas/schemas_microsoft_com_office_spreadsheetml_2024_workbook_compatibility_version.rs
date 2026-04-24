//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Version Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlwcv:version.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlwcv:CT_Version/xlwcv:version")]
pub struct Version {
  /// warnBelowVersion
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :warnBelowVersion
  #[sdk(attr(qname = ":warnBelowVersion"))]
  pub warn_below_version: Option<crate::simple_type::UInt32Value>,
  /// setVersion
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :setVersion
  #[sdk(attr(qname = ":setVersion"))]
  pub set_version: Option<crate::simple_type::UInt32Value>,
}
