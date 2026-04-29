//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ReadonlyRecommended Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is p1710:readonlyRecommended.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p1710:CT_ReadonlyRecommended/p1710:readonlyRecommended")]
pub struct ReadonlyRecommended {
  /// val
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
