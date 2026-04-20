//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ExternalCodeService Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlecs:externalCodeService.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlecs:CT_ExternalCodeService/xlecs:externalCodeService")]
pub struct ExternalCodeService {
  /// autoShow
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :autoShow
  #[sdk(attr(qname = ":autoShow"))]
  pub auto_show: Option<crate::simple_type::UInt32Value>,
  /// timeout
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :timeout
  #[sdk(attr(qname = ":timeout"))]
  pub timeout: Option<crate::simple_type::UInt32Value>,
}
