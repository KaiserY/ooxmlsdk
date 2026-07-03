//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Key Class.
pub type Key = crate::simple_type::StringValue;
/// Defines the Value Class.
pub type Value = crate::simple_type::StringValue;
/// Defines the RoamingProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pRoam:props")]
pub struct RoamingProperty {
  /// Defines the Key Class.
  #[sdk(text_child(qname = "pRoam:key"))]
  pub key: Key,
  /// Defines the Value Class.
  #[sdk(text_child(qname = "pRoam:value"))]
  pub value: Value,
}
