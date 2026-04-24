//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Decorative Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is adec:decorative.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "adec:CT_Decorative/adec:decorative")]
pub struct Decorative {
  /// val
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
