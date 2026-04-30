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
#[sdk(office2013, qname = "pRoam:CT_RoamingProperty/pRoam:props")]
pub struct RoamingProperty {
  /// _
  #[sdk(text_child(office2013, qname = "xsd:string/pRoam:key"))]
  pub key: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(office2013, qname = "xsd:string/pRoam:value"))]
  pub value: crate::simple_type::StringValue,
}
