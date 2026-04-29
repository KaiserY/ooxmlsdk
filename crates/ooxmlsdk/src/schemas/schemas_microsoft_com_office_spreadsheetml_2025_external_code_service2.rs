//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ExternalCodeServiceImageAsInput Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlecs2:externalCodeServiceImageAsInput.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlecs2:CT_ExternalCodeServiceImageAsInput/xlecs2:externalCodeServiceImageAsInput")]
pub struct ExternalCodeServiceImageAsInput {
  /// maxWidth
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :maxWidth
  #[sdk(attr(qname = ":maxWidth"))]
  pub max_width: Option<crate::simple_type::UInt32Value>,
  /// maxHeight
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :maxHeight
  #[sdk(attr(qname = ":maxHeight"))]
  pub max_height: Option<crate::simple_type::UInt32Value>,
}
